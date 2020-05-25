use std::fs::{create_dir, create_dir_all};
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};

use crate::revlog::RevLog;

#[derive(Debug, PartialEq)]
pub struct Repository {
    worktree: PathBuf,
    hg_dir: PathBuf,
}

const REPO_DIR: &str = "hg-rs";

impl Repository {
    pub fn new<P: AsRef<Path>>(worktree: P) -> Repository {
        Repository {
            worktree: worktree.as_ref().into(),
            hg_dir: worktree.as_ref().join(REPO_DIR),
        }
    }

    pub fn from_cwd() -> Result<Repository> {
        std::env::current_dir()?
            .ancestors()
            .find(|path| path.join(REPO_DIR).exists())
            .map(Repository::new)
            .context("Failed to find Mercurial repository.")
    }

    pub fn repo_path<P: AsRef<Path>>(&self, path: P) -> PathBuf {
        self.hg_dir.join(path)
    }

    // file_path("hello.txt") -> ~/Documents/hg-rs/hello.txt
    pub fn file_path<P: AsRef<Path>>(&self, path: P) -> PathBuf {
        self.worktree.join(path)
    }

    pub fn init(&self) -> Result<()> {
        create_dir(&self.hg_dir)?;
        create_dir(self.repo_path("cache"))?;
        create_dir_all(self.repo_path("store").join("data"))?;
        Ok(())
    }

    pub fn revlog<P: AsRef<Path>>(&self, path: P) -> Result<RevLog> {
        RevLog::for_file(self.hg_dir.join("store").join("data").join(path))
    }

    pub fn manifest_revlog(&self) -> Result<RevLog> {
        RevLog::new(
            self.hg_dir.join("store").join("00manifest.i"),
            self.hg_dir.join("store").join("00manifest.d"),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile;

    #[test]
    fn test_repo_path() {
        let repo = Repository::new("f");
        assert_eq!(repo.repo_path("g"), PathBuf::from("f/hg-rs/g"));
    }

    #[test]
    fn test_init() -> Result<()> {
        let base_dir = tempfile::tempdir()?;
        let base_dir = base_dir.path();
        let repo = Repository::new(base_dir);
        assert!(!base_dir.join(REPO_DIR).exists());
        repo.init()?;
        assert!(base_dir.join(REPO_DIR).exists());
        assert!(base_dir.join(REPO_DIR).join("cache").exists());
        Ok(())
    }
}
