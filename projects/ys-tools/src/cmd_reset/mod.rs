use clap::Args;
use std::env::current_dir;
use ys_types::{
    initialize::{DotYuanShenClient, YuanShenClient},
    IgnoreRules, ObjectID, ObjectProxy, ObjectStore, Commit, SnapShotTree, YsError,
};

#[derive(Debug, Args)]
pub struct YuanShenReset {
    /// 目标提交哈希或分支名
    target: String,
    #[arg(long)]
    no_refresh: bool,
    #[arg(long)]
    mixed: bool,
    #[arg(long)]
    soft: bool,
    /// 硬重置模式，同时重置工作目录
    #[arg(long)]
    hard: bool,
    #[arg(long)]
    merge: bool,
    #[arg(long)]
    keep: bool,
}

impl YuanShenReset {
    /// 执行 reset 命令，将当前分支重置到指定提交
    pub async fn reset(self) -> Result<(), YsError> {
        let dir = current_dir()?;
        let dot_rev = DotYuanShenClient::open(&dir)?;
        let store = ys_storage::LocalDotYuanShen::new(dir.join(".ys"))?;
        
        let target_commit_id: ObjectID = if dot_rev.branch_exists(&self.target)? {
            dot_rev.get_branch_id(&self.target)?
        } else {
            self.target.parse()?
        };
        
        let target_commit: Commit = store.get_typed(target_commit_id).await?;
        let target_tree: SnapShotTree = store.get_typed(target_commit.tree).await?;
        
        if self.hard {
            target_tree.write(&store, &dir).await?;
        }
        
        let current_branch: String = dot_rev.get_branch_name()?;
        dot_rev.set_branch_snapshot_id(&current_branch, target_commit_id)?;
        
        Ok(())
    }
}
