use std::fs::File;
use std::io::Read;
use std::path::Path;

use anyhow::Result;

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

// Append contents of file into revlog.
pub fn snap<P: AsRef<Path>>(path: P) -> Result<()> {
    let repo = Repository::from_cwd()?;
    let mut revlog = repo.revlog(&path)?;
    let mut file = File::open(path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    revlog.add_revision(&buffer)?;
    Ok(())
}

// Display contents of a revision.
pub fn debugdata<P: AsRef<Path>>(path: P, rev: u32) -> Result<()> {
    let repo = Repository::from_cwd()?;
    let mut revlog = repo.revlog(&path)?;
    let hunk = revlog.get_hunk(rev)?;
    print!("{}", String::from_utf8(hunk)?);
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
