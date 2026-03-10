use clap::Args;
use std::env::current_dir;
use ys_types::{
    initialize::{DotYuanShenClient, YuanShenClient},
    IgnoreRules, ObjectID, ObjectStore, Commit, SnapShotTree, YsError,
};

/// 孤儿分支命令参数
///
/// 用于创建一个没有历史记录的新分支，当前工作区内容会作为初始提交
#[derive(Debug, Args)]
pub struct YuanShenOrphan {
    /// 新分支的名称
    branch: String,
    /// 初始提交信息
    #[clap(short, long, default_value = "Initial commit")]
    message: String,
}

impl YuanShenOrphan {
    /// 执行孤儿分支创建操作
    ///
    /// 创建一个新的分支，该分支从一个全新的初始提交开始，没有父提交，
    /// 初始提交包含当前工作区的所有内容
    pub async fn orphan(self) -> Result<(), YsError> {
        let dir = current_dir()?;
        let dot_rev = DotYuanShenClient::open(&dir)?;
        let mut store = ys_storage::LocalDotYuanShen::new(dir.join(".ys"))?;
        let ignores: IgnoreRules = dot_rev.ignores()?;
        let directory = SnapShotTree::new(dir.as_path(), &ignores, &mut store)?;
        let directory_id = store.put_typed(&directory).await?;
        let snap = Commit {
            tree: directory_id,
            parents: Default::default(),
            extra: ys_types::SnapShotData { 
                kind: 0, 
                message: self.message, 
                tenants: Default::default() 
            },
        };
        let snap_id = store.put_typed(&snap).await?;
        dot_rev.set_branch_snapshot_id(&self.branch, snap_id)?;
        dot_rev.set_branch(&self.branch)
    }
}
