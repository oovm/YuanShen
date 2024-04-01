use crate::ObjectID;

pub struct TextFile {
    /// Text buffer reference
    pub string_id: ObjectID,
}

pub struct TextIncremental {
    pub reference: ObjectID,
    pub edits: Vec<ObjectID>,
}

pub struct TextEdit {}
