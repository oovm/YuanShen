use crate::storage::LocalDotYuanShen;
use super::*;


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
        let mut store = LocalDotYuanShen::new(self.join("store"))?;
        let directory = SnapShotTree::default();
        // let directory = store.put_typed(&directory).await?;
        // let snapshot = Commit {
        //     datetime: SystemTime::now(),
        //     parents: vec![],
        //     authors: Default::default(),
        // };
        // extra: SnapShotData { kind: 0, message: "Project initialized!".to_string(), authors: Default::default() },
        // let snapshot_id = store.put_typed(&snapshot).await?;
        // write_json(&snapshot_id, &root.join("branches").join(self.initial_branch.as_ref()))?;
        todo!();
        Ok(DotYuanShenClient { dot_root: root, dot_config: config })
    }
    fn generate_branches(&self) -> std::io::Result<()> {
        // Specify the current branch
        let mut file = File::options().create(true).write(true).open(self.join("branch"))?;
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
    fn get_branch_id(&self, branch: &str) -> Result<ObjectID, YsError>;

    fn calculate_branch_id(&self) -> Result<ObjectID, YsError> {
        let branch = self.get_branch_name()?;
        self.get_branch_id(&branch)
    }

    /// Get the current branch
    fn get_branch_name(&self) -> Result<String, YsError>;
    /// Set current branch to given name
    fn set_branch(&self, new: &str) -> Result<(), YsError>;

    /// Create a branch and set it's head to the current snapshot
    fn create_branch(&self, name: &str) -> Result<ObjectID, YsError>;
}

impl YuanShenClient for DotYuanShenClient {
    fn get_branch_id(&self, branch: &str) -> Result<ObjectID, YsError> {
        ObjectID::read_branch(&self.dot_root, branch)
    }
    fn calculate_branch_id(&self) -> Result<ObjectID, YsError> {
        let branch = self.get_branch_name()?;
        self.get_branch_id(&branch)
    }
    fn get_branch_name(&self) -> Result<String, YsError> {
        Ok(read_to_string(&self.dot_root.join("branch"))?)
    }
    fn set_branch(&self, new: &str) -> Result<(), YsError> {
        let branch_path = self.dot_root.join("branch");
        truncate_write(branch_path, new.as_bytes())?;
        Ok(())
    }

    fn create_branch(&self, name: &str) -> Result<ObjectID, YsError> {
        let path = self.dot_root.join("branches").join(name);
        if path.exists() {
            ObjectID::read_branch(&self.dot_root, name)
        }
        else {
            let snapshot_id = self.calculate_branch_id()?;
            truncate_write(path, snapshot_id.to_string().as_bytes())?;
            Ok(snapshot_id)
        }
    }
}

impl DotYuanShenClient {
    pub fn set_branch_snapshot_id(&self, branch: &str, object_id: ObjectID) -> Result<(), YsError> {
        write_json(&object_id, &self.dot_root.join("branches").join(&branch))
    }

    /// Checks whether a branch with a given name exists
    pub fn branch_exists(&self, branch: &str) -> Result<bool, YsError> {
        Ok(try_exists(self.dot_root.join("branches").join(&branch))?)
    }

    pub fn store(&self) -> Result<LocalDotYuanShen, YsError> {
        Ok(LocalDotYuanShen::new(self.dot_root.clone())?)
    }

    pub fn ignores(&self) -> Result<IgnoreRules, YsError> {
        Ok(read_json(&self.dot_root.join("ignores"))?)
    }
}
