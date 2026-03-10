use clap::Args;
use std::env::current_dir;
use ys_types::{
    initialize::{DotYuanShenClient, YuanShenClient},
    ObjectID, ObjectProxy, Commit, SnapShotTree, YsError,
};

/// 差异比较命令参数
///
/// 用于比较当前分支与指定分支之间的差异
#[derive(Debug, Args)]
pub struct YuanShenDifference {
    /// 要比较的目标分支名称
    branch: String,
}

impl YuanShenDifference {
    /// 执行差异比较操作
    ///
    /// 比较当前分支与指定分支之间的文件差异
    pub async fn difference(self) -> Result<(), YsError> {
        let dir = current_dir()?;
        let dot_rev = DotYuanShenClient::open(&dir)?;
        let store = ys_storage::LocalDotYuanShen::new(dir.join(".ys"))?;
        let that_branch = self.branch.as_ref();
        let this_branch: String = dot_rev.get_branch_name()?;
        if !dot_rev.branch_exists(&that_branch)? {
            return Err(YsError::invalid_object(format!("no branch named {} exists", that_branch)));
        }
        let this_tip: ObjectID = dot_rev.get_branch_id(&this_branch)?;
        let that_tip: ObjectID = dot_rev.get_branch_id(&that_branch)?;
        let that_snapshot: Commit = store.get_typed(that_tip).await?;
        let that_branch_directory = store.get_typed(that_snapshot.tree).await?;
        let this_snapshot: Commit = store.get_typed(this_tip).await?;
        let this_branch_directory: SnapShotTree =
            store.get_typed(this_snapshot.tree).await?;
        let diff = &this_branch_directory.difference(&that_branch_directory);
        println!("{diff}");
        Ok(())
    }
}
