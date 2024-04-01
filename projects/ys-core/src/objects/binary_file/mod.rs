use crate::ObjectID;

pub struct BinaryFile {
    /// Buffer reference
    pub buffer_id: ObjectID,
}

pub struct BinaryIncremental {
    pub reference: ObjectID,
    pub edits: Vec<ObjectID>,
}

pub struct BinaryEdit {}
