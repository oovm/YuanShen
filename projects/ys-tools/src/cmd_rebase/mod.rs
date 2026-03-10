use clap::Args;
use std::collections::BTreeSet;
use std::env::current_dir;
use ys_types::{
    initialize::{DotYuanShenClient, YuanShenClient},
    ObjectID, ObjectProxy, Commit, SnapShotData, YsError,
};

/// Rebase 命令参数
#[derive(Debug, Args)]
pub struct YuanShenRebase {
    /// 要变基到的目标分支名称
    #[clap(long)]
    branch: String,
}

impl YuanShenRebase {
    /// 执行 Rebase 命令，将当前分支变基到目标分支
    /// 
    /// # 功能说明
    /// - 查找当前分支和目标分支的共同祖先
    /// - 收集当前分支从共同祖先之后的所有提交
    /// - 将这些提交逐个重新应用到目标分支的最新提交之上（每个新提交使用原提交的 tree 和 message，但父提交更新）
    /// - 将当前分支指针更新到最后一个重新应用的提交
    pub async fn rebase(self) -> Result<(), YsError> {
        let dir = current_dir()?;
        let dot_rev = DotYuanShenClient::open(&dir).unwrap();
        let store = ys_storage::LocalDotYuanShen::new(dir.join(".ys")).unwrap();
        let current_branch: String = dot_rev.get_branch_name().unwrap();
        let current_tip: ObjectID = dot_rev.get_branch_id(&current_branch)?;
        let target_tip: ObjectID = dot_rev.get_branch_id(&self.branch)?;

        let base = self.find_common_ancestor(&store, current_tip, target_tip).await?;
        let commits = self.collect_commits(&store, current_tip, base).await?;
        let new_tip = self.apply_commits(&store, target_tip, &commits).await?;
        dot_rev.set_branch_snapshot_id(&current_branch, new_tip)
    }

    /// 查找两个提交的共同祖先
    /// 
    /// # 参数
    /// - store: 对象存储
    /// - a: 第一个提交 ID
    /// - b: 第二个提交 ID
    /// 
    /// # 返回
    /// - 共同祖先的提交 ID
    async fn find_common_ancestor(&self, store: &ys_storage::LocalDotYuanShen, a: ObjectID, b: ObjectID) -> Result<ObjectID, YsError> {
        if a == b {
            return Ok(a);
        }

        let mut visited_a: BTreeSet<ObjectID> = BTreeSet::new();
        let mut queue_a: Vec<ObjectID> = vec![a];
        let mut visited_b: BTreeSet<ObjectID> = BTreeSet::new();
        let mut queue_b: Vec<ObjectID> = vec![b];

        loop {
            if queue_a.is_empty() && queue_b.is_empty() {
                return Err(YsError::Other("No common ancestor found".to_string()));
            }

            if let Some(commit_id) = queue_a.pop() {
                if visited_b.contains(&commit_id) {
                    return Ok(commit_id);
                }
                if visited_a.insert(commit_id) {
                    if let Ok(commit) = store.get_typed::<Commit>(commit_id).await {
                        for parent in commit.parents {
                            queue_a.push(parent);
                        }
                    }
                }
            }

            if let Some(commit_id) = queue_b.pop() {
                if visited_a.contains(&commit_id) {
                    return Ok(commit_id);
                }
                if visited_b.insert(commit_id) {
                    if let Ok(commit) = store.get_typed::<Commit>(commit_id).await {
                        for parent in commit.parents {
                            queue_b.push(parent);
                        }
                    }
                }
            }
        }
    }

    /// 收集从 start 到 base 之间的所有提交（不包括 base）
    /// 
    /// # 参数
    /// - store: 对象存储
    /// - start: 起始提交 ID
    /// - base: 基础提交 ID（共同祖先）
    /// 
    /// # 返回
    /// - 提交列表，按从旧到新的顺序排列
    async fn collect_commits(&self, store: &ys_storage::LocalDotYuanShen, start: ObjectID, base: ObjectID) -> Result<Vec<Commit>, YsError> {
        let mut commits: Vec<Commit> = Vec::new();
        let mut current = start;

        while current != base {
            let commit = store.get_typed::<Commit>(current).await?;
            commits.push(commit.clone());
            if commit.parents.is_empty() {
                break;
            }
            current = *commit.parents.iter().next().unwrap();
        }

        commits.reverse();
        Ok(commits)
    }

    /// 将收集到的提交逐个应用到目标提交之上
    /// 
    /// # 参数
    /// - store: 对象存储
    /// - target: 目标提交 ID
    /// - commits: 要应用的提交列表
    /// 
    /// # 返回
    /// - 最后一个应用的提交的 ID
    async fn apply_commits(&self, store: &ys_storage::LocalDotYuanShen, target: ObjectID, commits: &[Commit]) -> Result<ObjectID, YsError> {
        let mut current_tip = target;

        for commit in commits {
            let mut parents = BTreeSet::new();
            parents.insert(current_tip);

            let new_commit = Commit {
                tree: commit.tree,
                parents,
                extra: SnapShotData {
                    kind: commit.extra.kind,
                    message: commit.extra.message.clone(),
                    tenants: commit.extra.tenants.clone(),
                },
            };

            current_tip = store.put_typed(&new_commit).await?;
        }

        Ok(current_tip)
    }
}
