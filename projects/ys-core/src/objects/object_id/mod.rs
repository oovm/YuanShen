use std::hash::{Hash, Hasher};
use super::*;
use crate::{

    utils::{read_json, write_json, WriteHashID},
};

mod convert;
#[cfg(test)]
mod tests;


/// 256 位对象 ID
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct ObjectID {
   pub(crate) hash256: blake3::Hash,
}

impl Hash for ObjectID {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.hash256.hash(state)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct BranchJson {
    tree_id: String,
}

impl ObjectID {
    pub fn read_branch(dot_ys: &Path, name: &str) -> Result<Self, YsError> {
        let file = dot_ys.join("branches").join(name);
        let json = read_json::<BranchJson>(&file)?;
        Ok(Self { hash256: blake3::Hash::from_hex(&json.tree_id)? })
    }
    pub fn write_branch(&self, dot_ys: &Path, name: &str) -> Result<(), YsError> {
        let file = dot_ys.join("branches").join(name);
        let json = BranchJson { tree_id: self.hash256.to_string() };
        write_json(&json, &file)
    }
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
        self.hash256.write_hash_id(f)
    }
}

impl Debug for ObjectID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}
