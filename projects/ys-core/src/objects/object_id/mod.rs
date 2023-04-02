use blake3::Hash;
use serde::{Deserialize, Serialize};

use std::{
    fmt::{Debug, Display},
    fs::File,
    io::Read,
    path::Path,
};
use serde::{Deserializer, Serializer};

#[cfg(test)]
mod tests;
mod convert;

/// 256 位对象 ID
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct ObjectID {
    hash256: Hash,
}

impl Ord for ObjectID {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.hash256.as_bytes().cmp(other.hash256.as_bytes())
    }
}

impl PartialOrd for ObjectID {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.hash256.as_bytes().partial_cmp(other.hash256.as_bytes())
    }
}

impl Display for ObjectID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for byte in self.hash256.as_bytes() {
            write!(f, "{:02x}", byte)?;
        }
        Ok(())
    }
}

impl Debug for ObjectID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

