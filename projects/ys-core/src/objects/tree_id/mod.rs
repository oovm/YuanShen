use blake3::Hash;
use std::{
    collections::BTreeMap,
    fmt::{Display, Formatter},
    path::Path,
};
use crate::traits::WriteHashID;

#[derive(Copy, Clone, Debug)]
pub struct TreeID {
    hash256: Hash,
}



impl Display for TreeID {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.hash256.write_hash_id(f)
    }
}

impl TreeID {
    pub fn read_branch(dot_ys: &Path, name: &str) {
        let file = dot_ys.join("branches").join(name);
    }
    pub fn write_branch(&self, dot_ys: &Path, name: &str) {
        let mut map: BTreeMap<&'static str, String> = BTreeMap::default();
        map.insert("tree-id", self.to_string())
    }
}
