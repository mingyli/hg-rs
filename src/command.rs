use std::fs::{File, OpenOptions};
use std::io::Read;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use chrono::Utc;

use crate::changeset::Changeset;
use crate::dirstate::{Entry, Status};
use crate::manifest::Manifest;
use crate::repository::Repository;

// Initialize a repository.
pub fn init() -> Result<()> {
    let repo = Repository::new(".");
    repo.init()?;
    Ok(())
}

// TODO: Handle multiple heads after merges are implemented.
pub fn heads() -> Result<()> {
    let repo = Repository::from_cwd()?;
    let mut changelog = repo.changelog_revlog()?;
    let size = changelog.size()?;
    let hunk = changelog.get_last_hunk()?;
    let changeset: Changeset = bincode::deserialize(&hunk)?;
    println!("{}", size);
    print!("{}", changeset);
    Ok(())
}

pub fn log() -> Result<()> {
    let repo = Repository::from_cwd()?;
    let mut changelog = repo.changelog_revlog()?;
    let size = changelog.size()?;
    for rev in (0..size).rev() {
        let hunk = changelog.get_hunk(rev)?;
        let changeset: Changeset = bincode::deserialize(&hunk)?;
        println!(
            "changeset: {}:{}",
            rev,
            hex::encode(changeset.manifest_nodeid)
        );
        println!("user:      {}", changeset.committer);
        if let Some(time) = changeset.time {
            println!("date:      {}", time);
        }
        println!("summary:   {}", changeset.message);
        println!();
    }
    Ok(())
}

// Dump the contents of an index file.
pub fn debug_index<P: AsRef<Path>>(path: P) -> Result<()> {
    let repo = Repository::from_cwd()?;
    let mut revlog = repo.revlog(path)?;
    revlog.debug_index()?;
    Ok(())
}

pub fn debug_changelog_index() -> Result<()> {
    let repo = Repository::from_cwd()?;
    let mut changelog = repo.changelog_revlog()?;
    changelog.debug_index()?;
    Ok(())
}

// Dump the contents of the manifest's index file.
pub fn debug_manifest_index() -> Result<()> {
    let repo = Repository::from_cwd()?;
    let mut revlog = repo.manifest_revlog()?;
    revlog.debug_index()?;
    Ok(())
}

// Append contents of file into revlog.
pub fn snapshot<P: AsRef<Path>>(path: P) -> Result<()> {
    let repo = Repository::from_cwd()?;
    let mut revlog = repo.revlog(&path)?;
    let mut file = File::open(&path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    let record = revlog.add_revision(&buffer)?;

    let mut manifest_revlog = repo.manifest_revlog()?;
    let mut manifest: Manifest = match manifest_revlog.get_last_hunk() {
        Ok(hunk) => bincode::deserialize(&hunk)?,
        Err(_) => Manifest::default(),
    };
    manifest.entries.insert(path.as_ref().into(), record.hash);
    manifest_revlog.add_revision(&bincode::serialize(&manifest)?)?;
    Ok(())
}

// Display contents of a revision.
pub fn debug_data<P: AsRef<Path>>(path: P, rev: u32) -> Result<()> {
    let repo = Repository::from_cwd()?;
    let mut revlog = repo.revlog(&path)?;
    let hunk = revlog.get_hunk(rev)?;
    print!("{}", String::from_utf8(hunk)?);
    Ok(())
}

pub fn debug_manifest_data(rev: u32) -> Result<()> {
    let repo = Repository::from_cwd()?;
    let mut manifest_revlog = repo.manifest_revlog()?;
    let hunk = manifest_revlog.get_hunk(rev)?;
    let manifest: Manifest = bincode::deserialize(&hunk)?;
    print!("{}", manifest);
    Ok(())
}

pub fn debug_changelog_data(rev: u32) -> Result<()> {
    let repo = Repository::from_cwd()?;
    let mut changelog = repo.changelog_revlog()?;
    let hunk = changelog.get_hunk(rev)?;
    let changeset: Changeset = bincode::deserialize(&hunk)?;
    print!("{}", changeset);
    Ok(())
}

pub fn add<P: AsRef<Path>>(path: P) -> Result<()> {
    use std::os::unix::fs::PermissionsExt;

    let repo = Repository::from_cwd()?;
    let mut dirstate = repo.dirstate()?;
    let file = OpenOptions::new().read(true).open(&path)?;
    let metadata = file.metadata()?;
    let entries = dirstate.mut_entries();
    entries.entry(path.as_ref().into()).or_insert(Entry {
        status: Status::Added,
        mode: metadata.permissions().mode(),
        size: metadata.len(),
        mtime: metadata.modified()?,
    });
    repo.commit_dirstate(dirstate)?;
    Ok(())
}

pub fn status() -> Result<()> {
    let repo = Repository::from_cwd()?;
    let mut dirstate = repo.dirstate()?;
    let entries = dirstate.mut_entries();
    for dir_entry in std::fs::read_dir(".")?
        .filter_map(Result::ok)
        .filter(|dir_entry| {
            !dir_entry
                .file_name()
                .to_str()
                .map(|s| s.starts_with("."))
                .unwrap_or(false)
        })
    {
        let path: PathBuf = dir_entry
            .path()
            .file_name()
            .context("Could not get file name.")?
            .into();
        let status_symbol = if let Some(entry) = entries.get(&path) {
            match entry.status {
                Status::Added => "A",
                Status::Merged | Status::Removed => unimplemented!(),
                Status::Normal => {
                    let metadata = dir_entry.metadata()?;
                    if entry.mtime == metadata.modified()? && entry.size == metadata.len() {
                        "C"
                    } else {
                        "M"
                    }
                }
            }
        } else {
            "?"
        };
        println!("{} {}", status_symbol, path.display());
    }

    Ok(())
}

pub fn commit(message: &str) -> Result<()> {
    let repo = Repository::from_cwd()?;
    let mut changelog = repo.changelog_revlog()?;
    let mut changeset: Changeset = changelog
        .get_last_hunk()
        .and_then(|hunk| bincode::deserialize(&hunk).map_err(anyhow::Error::from))
        .unwrap_or_default();

    let mut manifest_revlog = repo.manifest_revlog()?;
    let mut manifest: Manifest = manifest_revlog
        .get_last_hunk()
        .and_then(|hunk| bincode::deserialize(&hunk).map_err(anyhow::Error::from))
        .unwrap_or_default();

    // Update manifest with committed files.
    let mut dirstate = repo.dirstate()?;
    let mut commitable_files = dirstate.committable_files();
    for (path, _entry) in &commitable_files {
        // Update revlog of each file.
        // TODO: Defer writing to revlogs until end, when we actually know the ChangeSetId.
        let mut revlog = repo.revlog(&path)?;
        let mut file = File::open(&path)?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;
        let record = revlog.add_revision(&buffer)?;
        manifest.entries.insert(path.into(), record.hash);
    }

    // Update changelog with newest changeset.
    let record = manifest_revlog.add_revision(&bincode::serialize(&manifest)?)?;
    changeset.manifest_nodeid = record.hash;
    changeset.message = message.to_string();
    changeset.committer = "mingyli34@gmail.com".to_string();
    changeset.changed_files = commitable_files
        .iter()
        .map(|(path, _entry)| PathBuf::clone(path))
        .collect();
    changeset.time = Some(Utc::now());
    let record = changelog.add_revision(&bincode::serialize(&changeset)?)?;

    // Update dirstate with newest data.
    for (path, entry) in &mut commitable_files {
        use std::os::unix::fs::PermissionsExt;
        let metadata = std::fs::metadata(path)?;
        entry.status = Status::Normal;
        entry.size = metadata.len();
        entry.mode = metadata.permissions().mode();
    }
    dirstate.parent1_hash = record.hash;
    repo.commit_dirstate(dirstate)?;

    Ok(())
}

pub fn debug_dirstate() -> Result<()> {
    let repo = Repository::from_cwd()?;
    let dirstate = repo.dirstate()?;
    dirstate.debug()?;
    Ok(())
}
