use std::fs::File;
use std::io::Read;
use std::path::Path;

use anyhow::Result;

use crate::changeset::Changeset;
use crate::manifest::Manifest;
use crate::repository::Repository;

// Initialize a repository.
pub fn init() -> Result<()> {
    let repo = Repository::new(".");
    repo.init()?;
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

pub fn commit(message: &str) -> Result<()> {
    let repo = Repository::from_cwd()?;
    let mut changelog = repo.changelog_revlog()?;
    let mut changeset: Changeset = match changelog.get_last_hunk() {
        Ok(hunk) => bincode::deserialize(&hunk)?,
        Err(_) => Changeset::default(),
    };

    let mut manifest_revlog = repo.manifest_revlog()?;
    let mut manifest: Manifest = match manifest_revlog.get_last_hunk() {
        Ok(hunk) => bincode::deserialize(&hunk)?,
        Err(_) => Manifest::default(),
    };

    // TODO: Use paths in dirstate?.
    for path in &["hello.txt", "README.md"] {
        let mut revlog = repo.revlog(&path)?;
        let mut file = File::open(&path)?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;
        let record = revlog.add_revision(&buffer)?;
        manifest.entries.insert(path.into(), record.hash);
    }

    let record = manifest_revlog.add_revision(&bincode::serialize(&manifest)?)?;
    changeset.manifest_nodeid = record.hash;
    changeset.message = message.to_string();
    changeset.committer = "mingyli34@gmail.com".to_string();
    changeset.changed_files = vec!["hello.txt".into(), "README.md".into()];
    use std::time::SystemTime;
    changeset.time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)?;
    changelog.add_revision(&bincode::serialize(&changeset)?)?;
    Ok(())
}

// Replace contents of file.
// pub fn checkout(rev: u64) -> Result<()> {
//     let repo = Repository::from_cwd()?;
//     let mut revlog = repo.revlog(&path)?;
//     let hunk = revlog.get_hunk(rev)?;
//     let mut file = OpenOptions::new().write(true).truncate(true).open(&path)?;
//     file.write_all(&hunk)?;
//     Ok(())
// }
