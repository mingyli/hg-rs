use serde::{Deserialize, Serialize};

pub const RECORD_SIZE: usize = 64; // bytes

#[derive(Serialize, Deserialize, Debug, Default, PartialEq)]
pub struct Record {
    pub hunk_offset: u64,
    pub hunk_length: u32,
    pub uncompressed_length: u32,
    pub base_revision: u32,
    pub link_revision: u32,
    pub parent1_revision: u32,
    pub parent2_revision: u32,
    pub hash: [u8; 32],
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum Hunk {
    Full(Vec<u8>),
    Delta,
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;

    #[test]
    fn test_ser() -> Result<()> {
        assert_eq!(std::mem::size_of::<Record>(), RECORD_SIZE);
        let record = Record::default();
        let buf = bincode::serialize(&record)?;
        assert_eq!(buf.len(), RECORD_SIZE);
        let ff: Record = bincode::deserialize(&buf)?;
        assert_eq!(record, ff);
        Ok(())
    }
}
