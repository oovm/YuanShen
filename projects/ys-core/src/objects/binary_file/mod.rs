use super::*;

#[derive(Copy, Clone, Debug)]
pub struct BinaryFile {
    /// Buffer reference
    pub buffer_id: ObjectID,
}

#[derive( Clone, Debug)]
pub struct BinaryIncremental {
    pub reference: ObjectID,
    pub edits: Vec<ObjectID>,
}

#[derive(Copy, Clone, Debug)]
pub struct BinaryEdit {}
