use super::*;
use crate::DirectoryEntry;
use tokio::fs::File;

/// A raw text file
/// A pointer to the string in [YuanShenClient]
///
/// Text must hash by [BLAKE3](https://blake3.io/)
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct StandaloneTextFile {
    /// Associated with a [String] type
    pub string_id: ObjectID,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct IncrementalTextFile {
    pub incremental_id: ObjectID,
}

/// A incremental text file
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TextIncrementalInfo {
    /// The old reference
    pub base: DirectoryEntry,
    /// The edits
    pub edits: Vec<TextEdit>,
}

/// The text edit information
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct TextEdit {}

impl StandaloneTextFile {
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
    /// Read the text file from [YuanShenClient]
    pub async fn read<O>(&self, store: &O) -> Result<TextIncrementalInfo, YsError>
    where
        O: YuanShenClient,
    {
        todo!()
    }
}

impl TextIncrementalInfo {
    pub async fn resolve<O>(&self, store: &O) -> Result<String, YsError>
    where
        O: YuanShenClient,
    {
        self.apply(self.base.read_string(store).await?)
    }
    pub fn apply(&self, text: String) -> Result<String, YsError> {
        todo!("{text}")
    }
}

impl DirectoryEntry {
    pub async fn read_string<O>(&self, store: &O) -> Result<String, YsError>
    where
        O: YuanShenClient,
    {
        match self {
            Self::Directory(_) => {
                panic!()
            }
            Self::StandaloneText(v) => v.read(store).await,
            Self::IncrementalText(v) => v.read(store).await?.resolve(store).await,
            Self::Subtree(_) => {
                panic!()
            }
        }
    }
}
