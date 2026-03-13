use clap::Args;
use std::{collections::BTreeSet, env::current_dir};
use ys_types::{
    Commit, ObjectID, ObjectProxy, SnapShotData, YsError,
    initialize::{DotYuanShenClient, YuanShenClient},
};

/// Squash 命令参数
#[derive(Debug, Args)]
pub struct YuanShenSquash {
    /// 需要合并的提交数量
    #[clap(short, long)]
    count: u64,
    /// 新提交的信息
    #[clap(short, long)]
    message: String,
}

impl YuanShenSquash {
    /// 执行 Squash 命令，将多个提交合并为一个
    ///
    /// # 功能说明
    /// - 从当前分支最新提交向前合并指定数量的提交
    /// - 新提交包含所有变更，使用最新提交的 tree
    /// - 新提交的父提交是被合并提交之前的那个提交
    pub async fn squash(self) -> Result<(), YsError> {
        let dir = current_dir()?;
        let dot_rev = DotYuanShenClient::open(&dir).unwrap();
        let store = ys_storage::LocalDotYuanShen::new(dir.join(".ys")).unwrap();
        let branch: String = dot_rev.get_branch_name().unwrap();
        let mut current_commit_id: ObjectID = dot_rev.get_branch_id(&branch)?;
        let mut commits: Vec<Commit> = Vec::new();

        for _ in 0..self.count {
            let commit: Commit = store.get_typed(current_commit_id).await.unwrap();
            commits.push(commit.clone());
            if commit.parents.is_empty() {
                break;
            }
            current_commit_id = *commit.parents.iter().next().unwrap();
        }

        if commits.is_empty() {
            return Ok(());
        }

        let earliest_commit = commits.last().unwrap();
        let latest_commit = commits.first().unwrap();
        let new_parents: BTreeSet<ObjectID> = earliest_commit.parents.clone();

        let snap = Commit {
            tree: latest_commit.tree,
            parents: new_parents,
            extra: SnapShotData { kind: 0, message: self.message, tenants: Default::default() },
        };
        let snap_id = store.put_typed(&snap).await?;
        dot_rev.set_branch_snapshot_id(&branch, snap_id)
    }
}
