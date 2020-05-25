use std::collections::HashMap;
use std::fmt;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Manifest {
    pub entries: HashMap<PathBuf, [u8; 20]>,
}

impl fmt::Display for Manifest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (path, hash) in &self.entries {
            writeln!(
                f,
                "{}\x00{}",
                path.to_str().expect("Failed to convert path to string."),
                hex::encode(hash)
            )?;
        }
        write!(f, "")
    }
}
