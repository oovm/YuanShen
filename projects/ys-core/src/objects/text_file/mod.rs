use crate::{storage::YuanShenClient, ObjectID, YsError};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct TextFile {
    /// A pointer to the string in [YuanShenClient]
    /// 
    /// Text must hash by [BLAKE3](https://blake3.io/)
    pub string_id: ObjectID,
}

pub struct TextIncremental {
    pub reference: ObjectID,
    pub edits: Vec<ObjectID>,
}

pub struct TextEdit {}

impl TextFile {
    pub async fn read<O: YuanShenClient>(&self, store: &O) -> Result<String, YsError> {
        store.get_string(*self).await
    }
}
