use std::fs::{create_dir, create_dir_all};
use std::path::{Path, PathBuf};

use anyhow::Result;

#[derive(Debug, PartialEq)]
pub struct Repository {
    worktree: PathBuf,
    hg_dir: PathBuf,
}

impl Repository {
    pub fn new<P: AsRef<Path>>(worktree: P) -> Repository {
        Repository {
            worktree: worktree.as_ref().into(),
            hg_dir: worktree.as_ref().join(".hg"),
        }
    }

    pub fn from_directory<P: AsRef<Path>>(path: P) -> Option<Repository> {
        path.as_ref()
            .ancestors()
            .find(|path| path.join(".hg").exists())
            .map(Repository::new)
    }

    pub fn repo_path<P: AsRef<Path>>(&self, path: P) -> PathBuf {
        self.hg_dir.join(path)
    }

    pub fn file_path<P: AsRef<Path>>(&self, path: P) -> PathBuf {
        self.worktree.join(path)
    }

    pub fn init(&self) -> Result<()> {
        create_dir(&self.hg_dir)?;
        create_dir(self.repo_path("cache"))?;
        create_dir_all(self.repo_path("store").join("data"))?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile;

    #[test]
    fn test_repo_path() {
        let repo = Repository::new("f");
        assert_eq!(repo.repo_path("g"), PathBuf::from("f/.hg/g"));
    }

    #[test]
    fn test_init() -> Result<()> {
        let base_dir = tempfile::tempdir()?;
        let base_dir = base_dir.path();
        let repo = Repository::new(base_dir);
        assert!(!base_dir.join(".hg").exists());
        repo.init()?;
        assert!(base_dir.join(".hg").exists());
        assert!(base_dir.join(".hg").join("cache").exists());
        Ok(())
    }

    #[test]
    fn test_from_directory() -> Result<()> {
        let base_dir = tempfile::tempdir()?;
        let base_dir = base_dir.path();
        let repo = Repository::from_directory(base_dir);
        assert_eq!(repo, None);
        let repo = Repository::new(base_dir);
        repo.init()?;
        let repo = Repository::from_directory(base_dir);
        assert!(repo.is_some());
        Ok(())
    }
}
