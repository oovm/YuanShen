use super::*;
use crate::{DirectoryEntry, YuanShenObject};
use serde::Deserialize;
use std::future::Future;
use tokio::fs::File;

pub trait YuanShenID {
    type Object: YuanShenObject;

    fn load<O>(&self, store: &O) -> impl Future<Output = Result<Self::Object, YsError>> 
    where
        O: YuanShenClient + Send + Sync;
}

/// A raw text file
/// A pointer to the string in [YuanShenClient]
///
/// Text must hash by [BLAKE3](https://blake3.io/)
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct StandaloneText {
    /// Associated with a [IncrementalTextFile] type
    pub file_id: ObjectID,
}

impl YuanShenID for StandaloneText {
    type Object = String;

    async fn load<O>(&self, store: &O) -> Result<String, YsError>
    where
        O: YuanShenClient,
    {
        store.get_string(*self).await
    }
}

pub struct TextIncremental {
    pub data_id: ObjectID,
}

impl YuanShenID for TextIncremental {
    type Object = IncrementalTextFile;

    async fn load<O>(&self, store: &O) -> Result<Self::Object, YsError>
    where
        O: YuanShenClient,
    {
        todo!()
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct IncrementalTextFile {
    /// The old file reference
    base: ObjectID,
    /// The edits
    edits: Vec<TextEdit>,
}

impl YuanShenObject for IncrementalTextFile {
    fn object_id(&self) -> ObjectID {
        let mut hasher = blake3::Hasher::default();
        hasher.update(self.base.hash256.as_bytes());
        for _ in &self.edits {
            // hasher.update(&edit.hash256.as_bytes());
        }
        hasher.finalize().into()
    }
}

/// The text edit information
#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TextEdit {}

impl StandaloneText {
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

impl IncrementalTextFile {
    /// Resolve the text data
    pub async fn resolve<O>(self, store: &O) -> Result<String, YsError>
    where
        O: YuanShenClient,
    {
        match self {
            Self::Standalone { text } => Ok(text),
            Self::Incremental { base, edits } => {
                let mut base = store.get_string(base).await?;
                for edit in edits {
                    edit.apply(&mut base)?
                }
                Ok(base)
            }
        }
    }
}

impl TextEdit {
    /// Read the text file from [YuanShenClient]
    pub fn apply(self, text: &mut String) -> Result<(), YsError> {
        todo!("{text}")
    }
}
