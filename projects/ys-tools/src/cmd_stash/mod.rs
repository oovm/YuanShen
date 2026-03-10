use clap::{Args, Subcommand};
use std::{
    collections::BTreeSet,
    env::current_dir,
    fs,
    path::{Path, PathBuf},
};
use ys_types::{
    initialize::{DotYuanShenClient, YuanShenClient},
    IgnoreRules, ObjectID, ObjectProxy, ObjectStore, Commit, SnapShotData, SnapShotTree, YsError,
};

/// Stash 命令子命令
#[derive(Debug, Subcommand)]
pub enum StashSubcommand {
    /// 保存当前工作区更改
    Save(SaveArgs),
    /// 恢复最新的 stash 并删除它
    Pop,
    /// 恢复最新的 stash 但不删除
    Apply,
    /// 列出所有保存的 stash
    List,
    /// 删除指定的 stash
    Drop(DropArgs),
}

/// 保存 stash 的参数
#[derive(Debug, Args)]
pub struct SaveArgs {
    /// stash 消息
    #[clap(short, long)]
    message: Option<String>,
}

/// 删除 stash 的参数
#[derive(Debug, Args)]
pub struct DropArgs {
    /// 要删除的 stash 索引（从 0 开始）
    index: usize,
}

/// Stash 命令主结构
#[derive(Debug, Args)]
pub struct YuanShenStash {
    #[clap(subcommand)]
    subcommand: StashSubcommand,
}

impl YuanShenStash {
    /// 执行 stash 操作
    pub async fn stash(self) -> Result<(), YsError> {
        let dir = current_dir()?;
        let dot_rev = DotYuanShenClient::open(&dir)?;
        let mut store = ys_storage::LocalDotYuanShen::new(dir.join(".ys"))?;
        let stash_dir = dir.join(".ys").join("stash");

        match self.subcommand {
            StashSubcommand::Save(args) => Self::save(&dir, &dot_rev, &mut store, &stash_dir, args.message).await?,
            StashSubcommand::Pop => Self::pop(&dir, &dot_rev, &mut store, &stash_dir).await?,
            StashSubcommand::Apply => Self::apply(&dir, &dot_rev, &mut store, &stash_dir).await?,
            StashSubcommand::List => Self::list(&stash_dir, &mut store).await?,
            StashSubcommand::Drop(args) => Self::drop(&stash_dir, args.index).await?,
        }

        Ok(())
    }

    /// 保存当前工作区为 stash
    async fn save(
        dir: &Path,
        dot_rev: &impl YuanShenClient,
        store: &mut impl ObjectStore,
        stash_dir: &Path,
        message: Option<String>,
    ) -> Result<(), YsError> {
        fs::create_dir_all(stash_dir)?;

        let branch: String = dot_rev.get_branch_name()?;
        let old_tip: ObjectID = dot_rev.get_branch_id(&branch)?;
        let ignores: IgnoreRules = dot_rev.ignores()?;

        let directory = SnapShotTree::new(dir, &ignores, store)?;
        let directory_id = store.put_typed(&directory).await?;

        let stash_commit = Commit {
            tree: directory_id,
            parents: vec![old_tip].into_iter().collect(),
            extra: SnapShotData {
                kind: 1,
                message: message.unwrap_or_else(|| "WIP stash".to_string()),
                tenants: BTreeSet::new(),
            },
        };

        let stash_id = store.put_typed(&stash_commit).await?;

        let stash_list = Self::load_stash_list(stash_dir)?;
        let mut new_list = stash_list;
        new_list.push(stash_id);
        Self::save_stash_list(stash_dir, &new_list)?;

        let old_commit: Commit = store.get_typed(old_tip).await?;
        let old_tree: SnapShotTree = store.get_typed(old_commit.tree).await?;
        old_tree.restore(dir, &ignores, store)?;

        println!("Saved stash: {}", stash_id);

        Ok(())
    }

    /// 恢复最新的 stash 并删除
    async fn pop(
        dir: &Path,
        dot_rev: &impl YuanShenClient,
        store: &mut impl ObjectStore,
        stash_dir: &Path,
    ) -> Result<(), YsError> {
        let mut stash_list = Self::load_stash_list(stash_dir)?;

        if stash_list.is_empty() {
            return Err(YsError::stash_empty());
        }

        let stash_id = stash_list.pop().unwrap();
        Self::save_stash_list(stash_dir, &stash_list)?;

        Self::apply_stash(dir, dot_rev, store, &stash_id).await?;

        Ok(())
    }

    /// 恢复最新的 stash 但不删除
    async fn apply(
        dir: &Path,
        dot_rev: &impl YuanShenClient,
        store: &mut impl ObjectStore,
        stash_dir: &Path,
    ) -> Result<(), YsError> {
        let stash_list = Self::load_stash_list(stash_dir)?;

        if stash_list.is_empty() {
            return Err(YsError::stash_empty());
        }

        let stash_id = stash_list.last().unwrap();
        Self::apply_stash(dir, dot_rev, store, stash_id).await?;

        Ok(())
    }

    /// 列出所有 stash
    async fn list(
        stash_dir: &Path,
        store: &mut impl ObjectStore,
    ) -> Result<(), YsError> {
        let stash_list = Self::load_stash_list(stash_dir)?;

        if stash_list.is_empty() {
            println!("No stashes saved.");
            return Ok(());
        }

        for (index, stash_id) in stash_list.iter().enumerate() {
            let commit: Commit = store.get_typed(*stash_id).await?;
            println!("stash@{}: {} - {}", index, stash_id, commit.extra.message);
        }

        Ok(())
    }

    /// 删除指定的 stash
    async fn drop(
        stash_dir: &Path,
        index: usize,
    ) -> Result<(), YsError> {
        let mut stash_list = Self::load_stash_list(stash_dir)?;

        if index >= stash_list.len() {
            return Err(YsError::stash_index_out_of_bounds());
        }

        let removed = stash_list.remove(index);
        Self::save_stash_list(stash_dir, &stash_list)?;

        println!("Dropped stash: {}", removed);

        Ok(())
    }

    /// 应用指定的 stash 到工作区
    async fn apply_stash(
        dir: &Path,
        dot_rev: &impl YuanShenClient,
        store: &mut impl ObjectStore,
        stash_id: &ObjectID,
    ) -> Result<(), YsError> {
        let commit: Commit = store.get_typed(*stash_id).await?;
        let ignores: IgnoreRules = dot_rev.ignores()?;
        let tree: SnapShotTree = store.get_typed(commit.tree).await?;
        tree.restore(dir, &ignores, store)?;

        println!("Applied stash: {}", stash_id);

        Ok(())
    }

    /// 加载 stash 列表
    fn load_stash_list(stash_dir: &Path) -> Result<Vec<ObjectID>, YsError> {
        let list_path = stash_dir.join("list.json");

        if !list_path.exists() {
            return Ok(Vec::new());
        }

        let content = fs::read_to_string(list_path)?;
        let list: Vec<String> = serde_json::from_str(&content)?;
        Ok(list.into_iter().map(|s| s.into()).collect())
    }

    /// 保存 stash 列表
    fn save_stash_list(stash_dir: &Path, list: &[ObjectID]) -> Result<(), YsError> {
        fs::create_dir_all(stash_dir)?;
        let list_path = stash_dir.join("list.json");

        let str_list: Vec<String> = list.iter().map(|id| id.to_string()).collect();
        let content = serde_json::to_string_pretty(&str_list)?;
        fs::write(list_path, content)?;

        Ok(())
    }
}
