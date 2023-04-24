use crate::utils::WriteHashID;
use blake3::Hash;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::{
    cmp::Ordering,
    fmt::{Display, Formatter},
};

#[derive(Copy, Clone, Debug, Eq)]
pub struct TreeID {
    pub(crate) hash256: Hash,
}

impl PartialEq<Self> for TreeID {
    fn eq(&self, other: &Self) -> bool {
        self.hash256.as_bytes().eq(other.hash256.as_bytes())
    }
}

impl PartialOrd<Self> for TreeID {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.hash256.as_bytes().partial_cmp(other.hash256.as_bytes())
    }
}

impl Ord for TreeID {
    fn cmp(&self, other: &Self) -> Ordering {
        self.hash256.as_bytes().cmp(other.hash256.as_bytes())
    }
}

impl Serialize for TreeID {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.to_string().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for TreeID {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match Hash::from_hex(&s) {
            Ok(o) => Ok(Self { hash256: o }),
            Err(e) => Err(serde::de::Error::custom(e)),
        }
    }
}

impl Display for TreeID {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.hash256.write_hash_id(f)
    }
}
