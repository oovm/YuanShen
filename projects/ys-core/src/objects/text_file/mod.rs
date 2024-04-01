use crate::{storage::ObjectProxy, ObjectID, YsError};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct TextFile {
    /// Text buffer reference
    pub string_id: ObjectID,
}

pub struct TextIncremental {
    pub reference: ObjectID,
    pub edits: Vec<ObjectID>,
}

pub struct TextEdit {}

impl TextFile {
    pub async fn read<O: ObjectProxy>(&self, store: &O) -> Result<String, YsError> {
        store.get_string(*self).await
    }
}
