use std::fs::{File, OpenOptions};
use std::io::{Read, Write};

use anyhow::Result;

use crate::repository::Repository;
use crate::revlog::RevLog;

// Initialize a repository.
pub fn init() -> Result<()> {
    let repo = Repository::new(".");
    repo.init()?;
    Ok(())
}

// Dump the contents of an index file.
pub fn debug_index() -> Result<()> {
    let repo = Repository::new(".");
    let mut revlog = RevLog::new(repo.file_path("hello.txt"))?;
    revlog.debug_index()?;
    Ok(())
}

// Append contents of file into revlog.
pub fn snap() -> Result<()> {
    let repo = Repository::new(".");
    let path = repo.file_path("hello.txt");
    let mut revlog = RevLog::new(&path)?;
    let mut file = File::open(&path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    revlog.add_revision(&buffer)?;
    Ok(())
}

// Display contents of a revision.
pub fn debugdata(rev: u64) -> Result<()> {
    let repo = Repository::new(".");
    let path = repo.file_path("hello.txt");
    let mut revlog = RevLog::new(&path)?;
    let hunk = revlog.get_hunk(rev)?;
    println!("{}", String::from_utf8(hunk)?);
    Ok(())
}

// Replace contents of file.
pub fn checkout(rev: u64) -> Result<()> {
    let repo = Repository::new(".");
    let path = repo.file_path("hello.txt");
    let mut revlog = RevLog::new(&path)?;
    let hunk = revlog.get_hunk(rev)?;
    let mut file = OpenOptions::new().write(true).truncate(true).open(&path)?;
    file.write_all(&hunk)?;
    Ok(())
}
