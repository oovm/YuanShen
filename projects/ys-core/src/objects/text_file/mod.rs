use crate::{YsError, YuanShenClient};
use crate::objects::ObjectID;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct TextFile {
    /// A pointer to the string in [YuanShenClient]
    /// 
    /// Text must hash by [BLAKE3](https://blake3.io/)
    pub string_id: ObjectID,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TextIncremental {
    pub reference: ObjectID,
    pub edits: Vec<ObjectID>,
}
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct TextEdit {}

impl TextFile {
    pub async fn read<O: YuanShenClient>(&self, store: &O) -> Result<String, YsError> {
        store.get_string(*self).await
    }
}
