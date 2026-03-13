use clap::Args;
use std::{collections::BTreeSet, env::current_dir};
use ys_types::{
    Commit, ObjectID, ObjectProxy, SnapShotData, YsError,
    initialize::{DotYuanShenClient, YuanShenClient},
};

/// Merge 命令参数
#[derive(Debug, Args)]
pub struct YuanShenMerge {
    /// 要合并的分支名称
    #[clap(long)]
    branch: String,
    /// 合并提交信息
    #[clap(short, long)]
    message: String,
}

impl YuanShenMerge {
    /// 执行 Merge 命令，合并另一个分支到当前分支
    ///
    /// # 功能说明
    /// - 检查是否可以 fast-forward（当前分支是要合并分支的祖先）
    /// - 如果可以 fast-forward，直接移动当前分支指针到要合并分支的最新提交
    /// - 如果不能 fast-forward，创建一个合并提交，包含两个父提交（当前分支和要合并的分支）
    /// - 合并提交的 tree 使用要合并分支的最新 tree
    pub async fn merge(self) -> Result<(), YsError> {
        let dir = current_dir()?;
        let dot_rev = DotYuanShenClient::open(&dir).unwrap();
        let store = ys_storage::LocalDotYuanShen::new(dir.join(".ys")).unwrap();
        let current_branch: String = dot_rev.get_branch_name().unwrap();
        let current_tip: ObjectID = dot_rev.get_branch_id(&current_branch)?;
        let other_tip: ObjectID = dot_rev.get_branch_id(&self.branch)?;

        if self.is_fast_forward(&store, current_tip, other_tip).await {
            dot_rev.set_branch_snapshot_id(&current_branch, other_tip)
        }
        else {
            self.create_merge_commit(&dot_rev, &store, &current_branch, current_tip, other_tip).await
        }
    }

    /// 检查是否可以 fast-forward 合并
    ///
    /// # 参数
    /// - store: 对象存储
    /// - current_tip: 当前分支的最新提交 ID
    /// - other_tip: 要合并分支的最新提交 ID
    ///
    /// # 返回
    /// - 如果可以 fast-forward，返回 true；否则返回 false
    async fn is_fast_forward(&self, store: &ys_storage::LocalDotYuanShen, current_tip: ObjectID, other_tip: ObjectID) -> bool {
        if current_tip == other_tip {
            return true;
        }

        let mut visited: BTreeSet<ObjectID> = BTreeSet::new();
        let mut queue: Vec<ObjectID> = vec![other_tip];

        while let Some(commit_id) = queue.pop() {
            if commit_id == current_tip {
                return true;
            }
            if !visited.insert(commit_id) {
                continue;
            }

            if let Ok(commit) = store.get_typed::<Commit>(commit_id).await {
                for parent in commit.parents {
                    queue.push(parent);
                }
            }
        }

        false
    }

    /// 创建合并提交
    ///
    /// # 参数
    /// - dot_rev: YuanShen 客户端
    /// - store: 对象存储
    /// - current_branch: 当前分支名称
    /// - current_tip: 当前分支的最新提交 ID
    /// - other_tip: 要合并分支的最新提交 ID
    async fn create_merge_commit(
        &self,
        dot_rev: &DotYuanShenClient,
        store: &ys_storage::LocalDotYuanShen,
        current_branch: &str,
        current_tip: ObjectID,
        other_tip: ObjectID,
    ) -> Result<(), YsError> {
        let other_commit: Commit = store.get_typed(other_tip).await?;
        let mut parents: BTreeSet<ObjectID> = BTreeSet::new();
        parents.insert(current_tip);
        parents.insert(other_tip);

        let snap = Commit {
            tree: other_commit.tree,
            parents,
            extra: SnapShotData { kind: 0, message: self.message.clone(), tenants: Default::default() },
        };
        let snap_id = store.put_typed(&snap).await?;
        dot_rev.set_branch_snapshot_id(current_branch, snap_id)
    }
}
