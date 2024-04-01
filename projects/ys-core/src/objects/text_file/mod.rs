use super::*;
use std::hash::{Hash, Hasher};

/// A raw text file
/// A pointer to the string in [YuanShenClient]
///
/// Text must hash by [BLAKE3](https://blake3.io/)
#[derive(Copy, Clone, Debug, PartialEq, Eq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub struct TextFile {
    /// Associated with a [TextIncrementalData] type
    pub file_id: ObjectID,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub struct TextIncrementalFile {
    pub data_id: ObjectID,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TextIncrementalData {
    /// The old file reference
    base: DirectoryEntry,
    /// The edits
    edits: Vec<TextEdit>,
}

impl YuanShenID for TextFile {
    type Object = String;

    async fn load<O>(&self, store: &O) -> Result<String, YsError>
    where
        O: YuanShenClient,
    {
        store.get_string(*self).await
    }
}

impl YuanShenID for TextIncrementalFile {
    type Object = TextIncrementalData;

    async fn load<O>(&self, store: &O) -> Result<Self::Object, YsError>
    where
        O: YuanShenClient,
    {
        todo!()
    }
}

impl Hash for TextIncrementalData {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self.base {
            DirectoryEntry::Directory(_) => unreachable!(),
            DirectoryEntry::TextStandalone(_) => {}
            DirectoryEntry::TextIncremental(_) => {}
            DirectoryEntry::Subtree(_) => unreachable!(),
        }
        for edit in self.edits.iter() {
            edit.hash(state);
        }
    }
}

impl YuanShenObject for TextIncrementalData {
    fn object_id(&self) -> ObjectID {
        ObjectHasher::hash(self)
    }
}

/// The text edit information
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TextEdit {}

impl TextFile {
    /// Read the text file from [YuanShenClient]
    pub async fn read<O>(&self, store: &O) -> Result<String, YsError>
    where
        O: YuanShenClient,
    {
        store.get_string(*self).await
    }
    /// Write the text buffer to [YuanShenClient]
    pub async fn write<O>(&self, text: &str, store: &O) -> Result<Self, YsError>
    where
        O: YuanShenClient,
    {
        store.put_string(text).await
    }
    /// Write the text file to [YuanShenClient]
    pub async fn write_file<O>(&self, file: &mut File, store: &O) -> Result<Self, YsError>
    where
        O: YuanShenClient,
    {
        store.put_string_file(file).await
    }
    /// Write the text file to [YuanShenClient]
    pub async fn write_path<O, P>(&self, file: &mut P, store: &O) -> Result<Self, YsError>
    where
        O: YuanShenClient,
        P: AsRef<Path>,
    {
        // TODO: ensure text file
        let mut file = File::open(file.as_ref()).await?;
        store.put_string_file(&mut file).await
    }
}

impl TextIncrementalData {
    /// Resolve the text data
    pub async fn resolve<O>(self, store: &O) -> Result<String, YsError>
    where
        O: YuanShenClient,
    {
        todo!()
        // match self {
        //     Self::Standalone { text } => Ok(text),
        //     Self::Incremental { base, edits } => {
        //         let mut base = store.get_string(base).await?;
        //         for edit in edits {
        //             edit.apply(&mut base)?
        //         }
        //         Ok(base)
        //     }
        // }
    }
}

impl TextEdit {
    /// Read the text file from [YuanShenClient]
    pub fn apply(self, text: &mut String) -> Result<(), YsError> {
        todo!("{text}")
    }
}
