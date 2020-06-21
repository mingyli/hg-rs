use std::collections::HashMap;
use std::fs::OpenOptions;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum Status {
    Normal,
    Merged,
    Added,
    Removed,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Entry {
    pub status: Status,
    pub mode: u32,
    pub size: u64,
    pub mtime: SystemTime,
}

#[derive(Serialize, Deserialize, Default, Debug, PartialEq)]
pub struct Dirstate {
    pub parent1_hash: [u8; 20],
    parent2_hash: [u8; 20],
    entries: HashMap<PathBuf, Entry>,
}

impl Dirstate {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Dirstate> {
        if path.as_ref().exists() {
            let file = OpenOptions::new().read(true).open(path)?;
            let dirstate: Dirstate = bincode::deserialize_from(file)?;
            Ok(dirstate)
        } else {
            Ok(Dirstate::default())
        }
    }

    pub fn write_to_file<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let file = OpenOptions::new().write(true).create(true).open(path)?;
        bincode::serialize_into(file, &self)?;
        Ok(())
    }

    pub fn entries(&self) -> &HashMap<PathBuf, Entry> {
        &self.entries
    }

    pub fn mut_entries(&mut self) -> &mut HashMap<PathBuf, Entry> {
        &mut self.entries
    }

    /// Return the files that should be included in the new commit.
    /// Qualifying files are
    /// - added,
    /// - removed,
    /// - merged, and
    /// - normal files with different modification time or size.
    pub fn committable_files(&mut self) -> Vec<(&PathBuf, &mut Entry)> {
        self.entries
            .iter_mut()
            .filter_map(|(path, entry)| {
                // .filter(|(path, entry)| {
                // let path = path.clone();
                match entry.status {
                    Status::Added | Status::Removed => Some((path, entry)),
                    Status::Normal => {
                        let file = OpenOptions::new().read(true).open(&path).ok()?;
                        let metadata = file.metadata().ok()?;
                        if metadata.len() != entry.size || metadata.modified().ok()? != entry.mtime
                        {
                            Some((path, entry))
                        } else {
                            None
                        }
                    }
                    Status::Merged => unimplemented!("Merge is unimplemented."),
                }
            })
            .collect()
    }

    pub fn debug(&self) -> Result<()> {
        for (name, entry) in self.entries.iter() {
            println!(
                "{:?}  {}  {}",
                entry.status,
                entry.size,
                name.to_str()
                    .context("Failed to convert PathBuf to string.")?
            );
        }
        Ok(())
    }
}
