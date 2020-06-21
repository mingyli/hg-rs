use serde::{de, Deserialize, Deserializer, Serialize, Serializer};

pub const RECORD_SIZE: usize = 64; // bytes
pub const NULLID_REVISION: u32 = u32::MAX;

#[derive(Serialize, Deserialize, Debug, Default, PartialEq)]
pub struct Record {
    pub hunk_offset: u64,
    pub hunk_length: u32,
    pub uncompressed_length: u32,
    pub base_revision: u32,
    pub link_revision: u32,
    #[serde(
        serialize_with = "serialize_option_u32",
        deserialize_with = "deserialize_option_u32"
    )]
    pub parent1_revision: Option<u32>,
    #[serde(
        serialize_with = "serialize_option_u32",
        deserialize_with = "deserialize_option_u32"
    )]
    pub parent2_revision: Option<u32>,
    pub hash: [u8; 20],
    pub padding: [u8; 12],
}

fn serialize_option_u32<S>(revision: &Option<u32>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_u32(revision.unwrap_or(NULLID_REVISION))
}

fn deserialize_option_u32<'de, D>(deserializer: D) -> Result<Option<u32>, D::Error>
where
    D: Deserializer<'de>,
{
    let v: u32 = de::Deserialize::deserialize(deserializer)?;
    if v == NULLID_REVISION {
        Ok(None)
    } else {
        Ok(Some(v))
    }
}

// TODO: Implement deltas.
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum Hunk {
    Snapshot(Vec<u8>),
    Delta,
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;

    #[test]
    fn test_ser() -> Result<()> {
        let record = Record::default();
        assert_eq!(bincode::serialized_size(&record)?, RECORD_SIZE as u64);
        let buf = bincode::serialize(&record)?;
        assert_eq!(buf.len(), RECORD_SIZE);
        let ff: Record = bincode::deserialize(&buf)?;
        assert_eq!(record, ff);
        Ok(())
    }
}
