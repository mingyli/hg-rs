use std::fs::File;
use std::io::Read;
use std::path::Path;

use anyhow::Result;

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

// Replace contents of file.
// pub fn checkout(rev: u64) -> Result<()> {
//     let repo = Repository::from_cwd()?;
//     let mut revlog = repo.revlog(&path)?;
//     let hunk = revlog.get_hunk(rev)?;
//     let mut file = OpenOptions::new().write(true).truncate(true).open(&path)?;
//     file.write_all(&hunk)?;
//     Ok(())
// }
