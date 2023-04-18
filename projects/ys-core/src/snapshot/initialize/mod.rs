use super::*;

/// `.ys` 文件夹
#[derive(Debug)]
pub struct DotYuanShen {
    root: PathBuf,
}
#[derive(Debug)]
pub struct InitializeConfig {
    pub current: PathBuf,
    pub initial_branch: Cow<'static, str>,
    pub ignores: IgnoreRules,
}

impl InitializeConfig {
    ///
    pub async fn generate(&self) -> Result<DotYuanShen, YsError> {
        let root = self.current.join(DOT_YUAN_SHEN);
        if read_dir(&root).is_ok() {
            return Ok(DotYuanShen { root });
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
        write_json(&snapshot_id, &root.join(BRANCHES_DIRECTORY).join(self.initial_branch.as_ref()))?;

        Ok(DotYuanShen { root })
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
impl DotYuanShen {
    /// 打开一个已经存在的 `.ys` 文件夹
    pub fn open(root: PathBuf) -> Result<Self, YsError> {
        if root.exists() {
            // TODO: check valid
            Ok(Self { root })
        }
        else {
            Err(YsError::path_error(std::io::Error::new(std::io::ErrorKind::NotFound, "Directory does not exist"), root))
        }
    }
}

impl DotYuanShen {
    /// Get the root path of the `.ys` folder
    pub fn root(&self) -> &Path {
        &self.root
    }

    /// Get the current branch
    pub fn get_branch(&self) -> Result<String, YsError> {
        Ok(read_to_string(&self.root.join(CURRENT_BRANCH_FILE))?)
    }

    /// Set current branch to given name
    pub fn set_branch(&self, new: &str) -> Result<(), YsError> {
        let mut file = File::options().write(true).truncate(true).open(&self.root.join(CURRENT_BRANCH_FILE))?;
        file.write(new.as_bytes())?;
        Ok(())
    }

    pub fn get_branch_snapshot_id(&self, branch: &str) -> Result<ObjectID, YsError> {
        read_json(&self.root.join(BRANCHES_DIRECTORY).join(&branch))
    }

    pub fn set_branch_snapshot_id(&self, branch: &str, object_id: ObjectID) -> Result<(), YsError> {
        write_json(&object_id, &self.root.join(BRANCHES_DIRECTORY).join(&branch))
    }

    pub fn current_snapshot_id(&self) -> Result<ObjectID, YsError> {
        let branch = self.get_branch()?;
        self.get_branch_snapshot_id(&branch)
    }

    pub fn create_branch(&self, new_branch: &str) -> Result<(), YsError> {
        if !self.branch_exists(&new_branch)? {
            let snapshot_id = self.current_snapshot_id()?;
            return write_json(&snapshot_id, &self.root.join(BRANCHES_DIRECTORY).join(&new_branch));
        }
        Ok(())
    }

    /// Checks whether a branch with a given name exists
    pub fn branch_exists(&self, branch: &str) -> Result<bool, YsError> {
        Ok(try_exists(self.root.join(BRANCHES_DIRECTORY).join(&branch))?)
    }

    pub fn store(&self) -> Result<LocalObjectStore, YsError> {
        Ok(LocalObjectStore::new(self.root.clone())?)
    }

    pub fn ignores(&self) -> Result<IgnoreRules, YsError> {
        Ok(read_json(&self.root.join("ignores"))?)
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
