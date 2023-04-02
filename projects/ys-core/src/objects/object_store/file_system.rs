use super::*;


#[derive(Debug, Clone)]
pub struct LocalObjectStore {
    root: PathBuf,
}

impl LocalObjectStore {
    pub fn new(root: PathBuf) -> Result<Self, std::io::Error> {
        if !try_exists(&root)? {
            tracing::info!("正在创建储存库: {:?}", root);
            create_dir(&root)?;
        }
        Ok(Self { root })
    }
}

impl ObjectStore for LocalObjectStore {
    type Error = std::io::Error;

    async fn has(&self, id: ObjectID) -> Result<bool, Self::Error> {
        tracing::info!("检查 {} 中是否存在 {:?}", id, self.root);
        let s: String = format!("{}", id);
        let subdir: &str = &s[0..2];
        let filename: &str = &s[2..];
        let path = self.root.join(format!("{}/{}", subdir, filename));
        std::fs::try_exists(path)
    }

    async fn read(&self, id: ObjectID) -> Result<Option<Vec<u8>>, Self::Error> {
        tracing::info!("怎在 {} 中读取 {:?}", id, self.root);
        let s: String = format!("{}", id);
        let subdir: &str = &s[0..2];
        let filename: &str = &s[2..];
        let path = self.root.join(format!("{}/{}", subdir, filename));
        match std::fs::File::options().read(true).open(path) {
            Ok(mut f) => {
                let mut v = Vec::new();
                f.read_to_end(&mut v)?;
                Ok(Some(v))
            }
            Err(err) => {
                if err.kind() == ErrorKind::NotFound {
                    Ok(None)
                } else {
                    Err(err)
                }
            }
        }
    }

    async fn insert(&mut self, object: &[u8]) -> Result<ObjectID, Self::Error> {
        let id: ObjectID = object.into();
        tracing::info!("正在插入 {} 到 {:?}", id, self.root);
        let s: String = format!("{}", id);
        let subdir: &str = &s[0..2];
        let filename: &str = &s[2..];
        let subdir_path = self.root.join(format!("{}", subdir));
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
        f.write(object)?;
        Ok(id)
    }
}

