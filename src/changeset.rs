use std::fmt;
use std::path::PathBuf;
use std::time::Duration;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default, PartialEq)]
pub struct Changeset {
    pub manifest_nodeid: [u8; 20],
    pub committer: String,
    pub message: String,
    pub time: Duration,
    pub changed_files: Vec<PathBuf>,
}

impl fmt::Display for Changeset {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}", hex::encode(self.manifest_nodeid))?;
        writeln!(f, "{}", self.committer)?;
        writeln!(f, "{}", self.time.as_secs())?;
        for path in &self.changed_files {
            writeln!(f, "{}", path.display())?;
        }
        writeln!(f, "\n{}", self.message)
    }
}
