use clap::Args;
use std::env::current_dir;
use ys_types::{
    initialize::{DotYuanShenClient, YuanShenClient},
    IgnoreRules, ObjectID, ObjectProxy, Commit, SnapShotData, SnapShotTree, YsError,
};

/// 提交命令参数
///
/// 用于将当前工作区的更改提交到 YuanShen 仓库
#[derive(Debug, Args)]
pub struct YuanShenCommit {
    /// 提交信息
    #[clap(short, long)]
    message: String,
    /// 提交作者
    #[clap(long)]
    author: Option<String>,
    /// 额外的提交数据
    #[clap(long)]
    data: Option<String>,
}

impl YuanShenCommit {
    /// 执行提交操作
    ///
    /// 创建当前工作区的快照，并将其提交到当前分支
    pub async fn commit(self) -> Result<(), YsError> {
        let dir = current_dir()?;
        let dot_rev = DotYuanShenClient::open(&dir)?;
        let mut store = ys_storage::LocalDotYuanShen::new(dir.join(".ys"))?;
        let branch: String = dot_rev.get_branch_name()?;
        let old_tip: ObjectID = dot_rev.get_branch_id(&branch)?;
        let ignores: IgnoreRules = dot_rev.ignores()?;
        let directory = SnapShotTree::new(dir.as_path(), &ignores, &mut store)?;
        let directory_id = store.put_typed(&directory).await?;
        let snap = Commit {
            tree: directory_id,
            parents: vec![old_tip].into_iter().collect(),
            extra: SnapShotData { kind: 0, message: self.message, tenants: Default::default() },
        };
        let snap_id = store.put_typed(&snap).await?;
        dot_rev.set_branch_snapshot_id(&branch, snap_id)
    }
}
