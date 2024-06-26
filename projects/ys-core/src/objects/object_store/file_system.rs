use super::*;
use crate::utils::vec_json;
use std::fs::{create_dir, try_exists};

/// 本地文件系统对象储存
#[derive(Debug, Clone)]
pub struct LocalObjectStore {
    root: PathBuf,
}

impl LocalObjectStore {
    /// 创建一个本地文件系统对象储存
    pub fn new(root: PathBuf) -> Result<Self, std::io::Error> {
        if !try_exists(&root)? {
            tracing::info!("正在创建储存库: {:?}", root);
            create_dir(&root)?;
        }
        Ok(Self { root })
    }
}

const HASH_HEADER_LENGTH: usize = 2;

impl ObjectStore for LocalObjectStore {
    async fn has(&self, id: ObjectID) -> Result<bool, YsError> {
        tracing::trace!("检查 {} 中是否存在 {:?}", id, self.root);
        let s: String = format!("{}", id);
        let dir: &str = &s[0..HASH_HEADER_LENGTH];
        let filename: &str = &s[HASH_HEADER_LENGTH..];
        let path = self.root.join(format!("{}/{}", dir, filename));
        Ok(std::fs::try_exists(path)?)
    }

    async fn get(&self, id: ObjectID) -> Result<Vec<u8>, YsError> {
        tracing::trace!("怎在 {} 中读取 {:?}", id, self.root);
        let s: String = format!("{}", id);
        let dir: &str = &s[0..HASH_HEADER_LENGTH];
        let filename: &str = &s[HASH_HEADER_LENGTH..];
        let path = self.root.join(format!("{}/{}", dir, filename));
        let mut f = std::fs::File::options().read(true).open(path)?;
        let mut v = Vec::new();
        f.read_to_end(&mut v)?;
        Ok(v)
    }



    async fn put(&mut self, object: &[u8]) -> Result<ObjectID, YsError> {
        let id: ObjectID = object.into();
        tracing::trace!("正在插入 {} 到 {:?}", id, self.root);
        let s: String = format!("{}", id);
        let sub: &str = &s[0..HASH_HEADER_LENGTH];
        let filename: &str = &s[HASH_HEADER_LENGTH..];
        let subdir_path = self.root.join(format!("{}", sub));
        let path = subdir_path.join(format!("{}", filename));
        if std::fs::try_exists(&path)? {
            tracing::info!("{:?} already exists", path);
            return Ok(id);
        }
        if !std::fs::try_exists(&subdir_path)? {
            tracing::info!("creating subdir path {:?} in {:?}", subdir_path, self.root);
            std::fs::create_dir(&subdir_path)?;
        }
        let mut f = File::options().create(true).write(true).open(path)?;
        f.write(&object)?;
        Ok(id)
    }

    async fn put_typed<I>(&mut self, object: &I) -> Result<ObjectID, YsError>
    where
        I: Serialize + Send + Sync,
    {
        let bytes = vec_json(object)?;
        self.put(&bytes).await
    }
}
