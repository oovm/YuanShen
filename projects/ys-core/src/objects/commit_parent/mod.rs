use crate::{objects::object_store::YuanShenObject, ObjectID};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Clone, Debug)]
pub struct CommitParent {
    pub tree: ObjectID,
    pub modifier: ObjectID,
}

impl Serialize for CommitParent {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(Some(2));
        map.serialize_entry("tree", &self.tree)?;
        map.serialize_entry("modifier", &self.modifier)?;
        map.end()
    }
}

impl<'de> Deserialize<'de> for CommitParent {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        todo!()
    }
}

impl YuanShenObject for CommitParent {}
