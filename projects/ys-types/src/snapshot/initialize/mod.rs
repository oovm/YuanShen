use super::*;
use crate::objects::{BranchJson, Commit, SnapShotData, ObjectID};
use crate::traits::YuanShenObject;
use std::{
    fs::{create_dir, create_dir_all, read_dir, File},
    io::Write,
};

/// `.ys` 文件夹
#[derive(Debug)]
pub struct DotYuanShenClient {
    dot_root: PathBuf,
    _dot_config: PathBuf,
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
            return Ok(DotYuanShenClient { dot_root: root, _dot_config: config });
        }
        create_dir_all(&root)?;
        self.generate_branches()?;
        self.generate_configs().await?;
        // 创建初始提交
        let directory = SnapShotTree::default();
        let directory_id = directory.object_id();
        let directory_path = root.join(&directory_id.to_string()[0..2]).join(&directory_id.to_string()[2..]);
        if let Some(parent) = directory_path.parent() {
            create_dir_all(parent)?;
        }
        let mut directory_file = File::create(directory_path)?;
        serde_json::to_writer_pretty(&mut directory_file, &directory)?;
        
        let snapshot = Commit {
            tree: directory_id,
            parents: Default::default(),
            extra: SnapShotData { kind: 0, message: "Project initialized!".to_string(), tenants: Default::default() },
        };
        let snapshot_id = snapshot.object_id();
        let snapshot_path = root.join(&snapshot_id.to_string()[0..2]).join(&snapshot_id.to_string()[2..]);
        if let Some(parent) = snapshot_path.parent() {
            create_dir_all(parent)?;
        }
        let mut snapshot_file = File::create(snapshot_path)?;
        serde_json::to_writer_pretty(&mut snapshot_file, &snapshot)?;
        
        let branch_file = root.join("branches").join(self.initial_branch.as_ref());
        write_json(&BranchJson { tree_id: snapshot_id.0 }, &branch_file)?;
        Ok(DotYuanShenClient { dot_root: root, _dot_config: config })
    }
    fn generate_branches(&self) -> std::io::Result<()> {
        // Specify the current branch
        let mut file = File::options().create(true).write(true).open(self.join("branch"))?;
        file.write(self.initial_branch.as_bytes())?;
        // Create the default branch
        create_dir(self.join("branches"))
    }
    async fn generate_configs(&self) -> Result<(), YsError> {
        let ignore = self.current.join(".ys.ignore");
        let mut file = File::options().create(true).write(true).open(ignore)?;
        file.write(self.ignores.glob.as_bytes())?;
        
        let ignores_in_dot_ys = self.join("ignores");
        write_json(&self.ignores, &ignores_in_dot_ys)?;
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
        Ok(Self { dot_root, _dot_config: dot_config })
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
        let branch_file = self.dot_root.join("branch");
        let mut file = File::options().create(true).write(true).truncate(true).open(branch_file)?;
        file.write_all(new.as_bytes())?;
        Ok(())
    }

    fn create_branch(&self, name: &str) -> Result<ObjectID, YsError> {
        let current_branch = self.get_branch_name()?;
        let current_tip = self.get_branch_id(&current_branch)?;
        self.set_branch_snapshot_id(name, current_tip)?;
        Ok(current_tip)
    }
}

impl DotYuanShenClient {
    pub fn set_branch_snapshot_id(&self, branch: &str, object_id: ObjectID) -> Result<(), YsError> {
        let json = BranchJson { tree_id: object_id.0 };
        write_json(&json, &self.dot_root.join("branches").join(&branch))
    }

    /// Checks whether a branch with a given name exists
    pub fn branch_exists(&self, branch: &str) -> Result<bool, YsError> {
        Ok(self.dot_root.join("branches").join(&branch).try_exists()?)
    }

    /// Lists all existing branches
    pub fn list_branches(&self) -> Result<Vec<String>, YsError> {
        let branches_dir = self.dot_root.join("branches");
        let mut branches = Vec::new();

        if !branches_dir.exists() {
            return Ok(branches);
        }

        for entry in read_dir(branches_dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() {
                if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                    branches.push(name.to_string());
                }
            }
        }

        Ok(branches)
    }

    pub fn ignores(&self) -> Result<IgnoreRules, YsError> {
        Ok(read_json(&self.dot_root.join("ignores"))?)
    }
}
