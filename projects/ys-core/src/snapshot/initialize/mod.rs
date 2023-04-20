use super::*;
use crate::YsErrorKind;
use std::str::FromStr;
/// `.ys` 文件夹
#[derive(Debug)]
pub struct DotYuanShenClient {
    dot_root: PathBuf,
    dot_config: PathBuf,
}

#[derive(Debug)]
pub struct InitializeConfig {
    pub current: PathBuf,
    pub initial_branch: Cow<'static, str>,
    pub ignores: IgnoreRules,
}

impl InitializeConfig {
    ///
    pub async fn generate(&self) -> Result<DotYuanShenClient, YsError> {
        let root = self.current.join(DOT_YUAN_SHEN);
        let config = self.current.join(".config").join("yuan-shen");
        if read_dir(&root).is_ok() {
            return Ok(DotYuanShenClient { dot_root: root, dot_config: config });
        }
        create_dir_all(&root)?;
        self.generate_branches()?;
        self.generate_configs()?;
        // 创建初始提交
        let mut store = LocalObjectStore::new(self.join("store"))?;
        let directory = SnapShotDirectory::default();
        let directory = store.insert_json(&directory).await?;
        let snapshot = SnapShot {
            directory,
            previous: BTreeSet::new(),
            data: SnapShotData { kind: 0, message: "Project initialized!".to_string(), authors: Default::default() },
        };
        let snapshot_id = store.insert_json(&snapshot).await?;
        write_json(&snapshot_id, &root.join("branches").join(self.initial_branch.as_ref()))?;

        Ok(DotYuanShenClient { dot_root: root, dot_config: config })
    }
    fn generate_branches(&self) -> std::io::Result<()> {
        // Specify the current branch
        let mut file = File::options().create(true).write(true).open(self.join(CURRENT_BRANCH_FILE))?;
        file.write(self.initial_branch.as_bytes())?;
        // Create the default branch
        create_dir(self.join("branches"))
    }
    fn generate_configs(&self) -> std::io::Result<()> {
        let ignore = self.current.join(".ys.ignore");
        let mut file = File::options().create(true).write(true).open(ignore)?;
        file.write(self.ignores.glob.as_bytes())?;
        Ok(())
    }
    fn join(&self, path: &str) -> PathBuf {
        self.current.join(DOT_YUAN_SHEN).join(path)
    }
}

impl DotYuanShenClient {
    /// Open a directory where the `.ys` folder exists
    pub fn open(path: &Path) -> Result<Self, YsError> {
        if path.is_file() {
            Err(YsError::path_error(
                std::io::Error::new(std::io::ErrorKind::NotFound, "The path must be a directory where the `.ys` folder exists"),
                path,
            ))?
        }
        let dot_root = path.join(DOT_YUAN_SHEN);
        if !dot_root.exists() {
            Err(YsError::path_error(std::io::Error::new(std::io::ErrorKind::NotFound, "Folder `.ys` does not exist"), path))?
        }
        let dot_config = path.join(".config").join("yuan-shen");
        // if !dot_config.exists() {
        //     Err(YsError::path_error(
        //         std::io::Error::new(std::io::ErrorKind::NotFound, "Folder `.config/yuan-shen` does not exist"),
        //         path,
        //     ))?
        // }
        Ok(Self { dot_root, dot_config })
    }
}

/// Describe the capabilities of the YuanShen client
pub trait YuanShenClient {
    /// Get the current branch
    fn get_branch(&self) -> Result<String, YsError>;
    /// Set current branch to given name
    fn set_branch(&self, new: &str) -> Result<ObjectID, YsError>;

    /// Create a branch and set it's head to the current snapshot
    fn create_branch(&self, name: &str) -> Result<ObjectID, YsError>;
}

impl YuanShenClient for DotYuanShenClient {
    fn get_branch(&self) -> Result<String, YsError> {
        Ok(read_to_string(&self.dot_root.join(CURRENT_BRANCH_FILE))?)
    }
    fn set_branch(&self, new: &str) -> Result<ObjectID, YsError> {
        let mut file = File::options().write(true).truncate(true).open(&self.dot_root.join(CURRENT_BRANCH_FILE))?;
        file.write(new.as_bytes())?;
        self.create_branch(new)
    }

    fn create_branch(&self, name: &str) -> Result<ObjectID, YsError> {
        let path = self.dot_root.join("branches").join(name);
        if path.exists() {
            match read_to_string(&path) {
                Ok(o) => ObjectID::from_str(&o),
                Err(e) => Err(YsErrorKind::IO { error: e, path: Some(path) })?,
            }
        }
        else {
            let snapshot_id = self.current_snapshot_id()?;
            let mut file = File::options().write(true).create(true).open(&path)?;
            match file.write_all(snapshot_id.to_string().as_bytes()) {
                Ok(_) => Ok(snapshot_id),
                Err(e) => Err(YsErrorKind::IO { error: e, path: Some(path) })?,
            }
        }
    }
}

impl DotYuanShenClient {
    pub fn get_branch_snapshot_id(&self, branch: &str) -> Result<ObjectID, YsError> {
        read_json(&self.dot_root.join("branches").join(&branch))
    }

    pub fn set_branch_snapshot_id(&self, branch: &str, object_id: ObjectID) -> Result<(), YsError> {
        write_json(&object_id, &self.dot_root.join("branches").join(&branch))
    }

    pub fn current_snapshot_id(&self) -> Result<ObjectID, YsError> {
        let branch = self.get_branch()?;
        self.get_branch_snapshot_id(&branch)
    }

    /// Checks whether a branch with a given name exists
    pub fn branch_exists(&self, branch: &str) -> Result<bool, YsError> {
        Ok(try_exists(self.dot_root.join("branches").join(&branch))?)
    }

    pub fn store(&self) -> Result<LocalObjectStore, YsError> {
        Ok(LocalObjectStore::new(self.dot_root.clone())?)
    }

    pub fn ignores(&self) -> Result<IgnoreRules, YsError> {
        Ok(read_json(&self.dot_root.join("ignores"))?)
    }
}

pub trait InsertJson {
    fn insert_json<A: Serialize + Send + Sync>(&mut self, thing: &A) -> impl Future<Output = Result<ObjectID, YsError>> + Send;

    fn read_json<A: for<'de> Deserialize<'de>>(&mut self, id: ObjectID) -> impl Future<Output = Result<A, YsError>> + Send;
}

impl InsertJson for LocalObjectStore {
    async fn insert_json<A: Serialize + Send + Sync>(&mut self, thing: &A) -> Result<ObjectID, YsError> {
        Ok(self.insert(&serde_json::to_vec_pretty(thing)?).await?)
    }

    async fn read_json<A: for<'de> Deserialize<'de>>(&mut self, object_id: ObjectID) -> Result<A, YsError> {
        let object = self.read(object_id).await?;
        Ok(serde_json::from_slice(&object)?)
    }
}

fn read_json<A: for<'de> Deserialize<'de>>(path: &Path) -> Result<A, YsError> {
    Ok(serde_json::from_reader(File::options().read(true).open(path)?)?)
}

fn write_json<A: Serialize>(thing: &A, path: &Path) -> Result<(), YsError> {
    let file = File::options().write(true).create(true).open(path)?;
    let mut ser = Serializer::with_formatter(file, PrettyFormatter::with_indent(b"    "));
    Ok(thing.serialize(&mut ser)?)
}
