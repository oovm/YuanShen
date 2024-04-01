use std::future::Future;
use std::pin::Pin;
use super::*;


/// 本地文件系统对象储存
#[derive(Debug, Clone)]
pub struct LocalDotYuanShen {
    root: PathBuf,
}

impl LocalDotYuanShen {
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

impl YuanShenClient for LocalDotYuanShen {
    async fn has(&self, id: ObjectID) -> Result<bool, YsError> {
        tracing::trace!("检查 {} 中是否存在 {:?}", id, self.root);
        let s: String = format!("{}", id);
        let dir: &str = &s[0..HASH_HEADER_LENGTH];
        let filename: &str = &s[HASH_HEADER_LENGTH..];
        let path = self.root.join(format!("{}/{}", dir, filename));
        Ok(std::fs::try_exists(path)?)
    }

    fn get_string(&self, _: StandaloneText) -> Pin<Box<dyn Future<Output=Result<String, YsError>> + Send>> {
        todo!()
    }


    async fn get_string_file(&self, _: StandaloneText, _: File) -> Result<File, YsError> {
        todo!()
    }
    // async fn get(&self, id: ObjectID) -> Result<Vec<u8>, YsError> {
    //     tracing::trace!("怎在 {} 中读取 {:?}", id, self.root);
    //     let s: String = format!("{}", id);
    //     let dir: &str = &s[0..HASH_HEADER_LENGTH];
    //     let filename: &str = &s[HASH_HEADER_LENGTH..];
    //     let path = self.root.join(format!("{}/{}", dir, filename));
    //     let mut f = std::fs::File::options().read(true).open(path)?;
    //     let mut v = Vec::new();
    //     f.read_to_end(&mut v)?;
    //     Ok(v)
    // }



    async fn put_string(&self, _: &str) -> Result<StandaloneText, YsError> {
        todo!()
    }

    async fn put_string_file(&self, _: &mut tokio::fs::File) -> Result<StandaloneText, YsError> {
        todo!()
    }

    async fn get_buffer(&self, _: StandaloneText) -> Result<String, YsError> {
        todo!()
    }

    async fn get_buffer_file(&self, _: StandaloneText, _: &mut File) -> Result<(), YsError> {
        todo!()
    }

    async fn put_buffer(&self, _: &str) -> Result<StandaloneText, YsError> {
        todo!()
    }

    async fn put_buffer_file(&self, _: &mut File) -> Result<StandaloneText, YsError> {
        todo!()
    }
}

impl LocalDotYuanShen {
    async fn put(&self, id: ObjectID, object: &[u8]) -> Result<ObjectID, YsError> {
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
        let mut f = File::options().create(true).write(true).open(path).await?;
        f.write(&object).await?;
        Ok(id)
    }
}
