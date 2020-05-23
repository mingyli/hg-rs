use std::fs::{File, OpenOptions};
use std::io::{Read, Seek, SeekFrom};
use std::path::Path;

use anyhow::{anyhow, Context, Result};

use crate::record::{Record, RECORD_SIZE};

pub struct RevLog {
    index: File,
    data: File,
}
impl RevLog {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<RevLog> {
        let index_path = path.as_ref().with_file_name(format!(
            "{}{}",
            path.as_ref()
                .file_name()
                .context("f")?
                .to_str()
                .context("F")?,
            ".i"
        ));
        let data_path = path.as_ref().with_file_name(format!(
            "{}{}",
            path.as_ref()
                .file_name()
                .context("f")?
                .to_str()
                .context("f")?,
            ".d"
        ));
        Ok(RevLog {
            index: OpenOptions::new()
                .read(true)
                .append(true)
                .create(true)
                .open(index_path)?,
            data: OpenOptions::new()
                .read(true)
                .append(true)
                .create(true)
                .open(data_path)?,
        })
    }

    // Number of revisions
    pub fn size(&self) -> Result<u64> {
        let num_bytes = self.index.metadata()?.len();
        if num_bytes % RECORD_SIZE as u64 == 0 {
            Ok(num_bytes / RECORD_SIZE as u64)
        } else {
            Err(anyhow!(
                "Index has {} bytes, which is not a multiple of {}.",
                num_bytes,
                RECORD_SIZE
            ))
        }
    }

    pub fn add_revision(&mut self, bytes: &[u8]) -> Result<()> {
        use std::io::Write;
        let new_rev = self.size()?;
        let record = if new_rev == 0 {
            Record {
                hunk_length: bytes.len() as u32,
                base_revision: new_rev as u32,
                ..Record::default()
            }
        } else {
            let latest_record = self.get_record(new_rev as u64 - 1)?;
            Record {
                hunk_length: bytes.len() as u32,
                base_revision: new_rev as u32,
                hunk_offset: latest_record.hunk_offset + latest_record.hunk_length as u64,
                ..Record::default()
            }
        };
        self.index.write_all(&bincode::serialize(&record)?)?;
        self.data.write_all(bytes)?;
        Ok(())
    }

    pub fn get_record(&mut self, rev: u64) -> Result<Record> {
        if rev >= self.size()? as u64 {
            return Err(anyhow!("Failed to seek revision {}.", rev));
        }
        self.index.seek(SeekFrom::Start(rev * RECORD_SIZE as u64))?;
        let record: Record = bincode::deserialize_from(&self.index)?;
        Ok(record)
    }

    pub fn get_hunk(&mut self, rev: u64) -> Result<Vec<u8>> {
        let record = self.get_record(rev)?;
        let mut buffer = vec![0u8; record.hunk_length as usize];
        self.data.seek(SeekFrom::Start(record.hunk_offset))?;
        self.data.read_exact(&mut buffer)?;
        Ok(buffer)
    }

    pub fn debug_index(&mut self) -> Result<()> {
        println!("rev offset length base");
        for rev in 0..self.size()? {
            let record = self.get_record(rev)?;
            println!(
                "{}   {}   {}   {}",
                rev, record.hunk_offset, record.hunk_length, record.base_revision
            );
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile;

    #[test]
    fn test_revlog() -> Result<()> {
        let base_dir = tempfile::tempdir()?;
        let mut revlog = RevLog::new(base_dir.path().join("hello"))?;
        assert_eq!(revlog.size()?, 0);
        revlog.add_revision(b"hello my bytes")?;
        assert_eq!(revlog.size()?, 1);
        revlog.add_revision(b"hello my other bytes")?;
        assert_eq!(revlog.size()?, 2);

        assert_eq!(
            revlog.get_record(0)?,
            Record {
                hunk_length: b"hello my bytes".len() as u32,
                base_revision: 0,
                ..Record::default()
            }
        );
        assert_eq!(
            revlog.get_record(1)?,
            Record {
                hunk_length: b"hello my other bytes".len() as u32,
                hunk_offset: b"hello my bytes".len() as u64,
                base_revision: 1,
                ..Record::default()
            }
        );
        assert!(revlog.get_record(2).is_err());
        assert_eq!(
            revlog.get_record(0)?,
            Record {
                hunk_length: b"hello my bytes".len() as u32,
                base_revision: 0,
                ..Record::default()
            }
        );
        Ok(())
    }
}
