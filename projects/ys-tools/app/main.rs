use clap::{ Parser, Subcommand};
use std::{env::current_dir, fmt::Debug, io::stdout};
use ys_types::{
    initialize::{DotYuanShenClient, YuanShenClient},
    IgnoreRules, ObjectID, ObjectProxy, Commit as YsCommit, SnapShotTree, YsError, GarbageCollect,
};
use ys_tools::*;

#[derive(Parser, Debug)]
struct YuanShen {
    #[clap(subcommand)]
    cmd: YsCommand,
}

#[derive(Debug, Subcommand)]
enum YsCommand {
    #[command(alias = "init")]
    Initialize(YuanShenInitialize),
    #[command(alias = "diff")]
    Difference(YuanShenDifference),
    Changes,
    Commit(YuanShenCommit),
    Squash(YuanShenSquash),
    Merge(YuanShenMerge),
    Rebase(YuanShenRebase),
    Reset(YuanShenReset),
    Orphan(YuanShenOrphan),
    Checkout(YuanShenCheckout),
    Branch(YuanShenBranch),
    #[command(alias = "gc")]
    GarbageCollect,
    #[command(external_subcommand)]
    External,
}

#[tokio::main]
pub async fn main() -> Result<(), YsError> {
    let args = YuanShen::parse();
    use YsCommand::*;
    match args.cmd {
        Initialize(init) => init.initialize().await?,
        Difference(diff) => diff.difference().await?,
        Branch(b) => b.branch().await?,
        Checkout(c) => c.checkout().await?,
        Changes => {
            let dir = current_dir()?;
            let dot_rev = DotYuanShenClient::open(&dir).unwrap();
            let mut store = ys_storage::LocalDotYuanShen::new(dir.join(".ys")).unwrap();
            let branch: String = dot_rev.get_branch_name().unwrap();
            let old_tip: ObjectID = dot_rev.get_branch_id(&branch).unwrap();
            let ignores: IgnoreRules = dot_rev.ignores().unwrap();
            let directory = SnapShotTree::new(dir.as_path(), &ignores, &mut store).unwrap();
            let snapshot: YsCommit = store.get_typed(old_tip).await.unwrap();
            let old_directory: SnapShotTree = store.get_typed(snapshot.tree).await.unwrap();
            serde_json::to_writer_pretty(stdout(), &old_directory.difference(&directory)).unwrap();
        }
        Commit(sub) => sub.commit().await.unwrap(),
        Squash(sub) => sub.squash().await.unwrap(),
        Merge(sub) => sub.merge().await.unwrap(),
        Rebase(sub) => sub.rebase().await.unwrap(),
        Reset(sub) => sub.reset().await?,
        Orphan(sub) => sub.orphan().await?,
        External => {}
        GarbageCollect => {
            let dir = current_dir()?;
            let store = ys_storage::LocalDotYuanShen::new(dir.join(".ys")).unwrap();
            let deleted_count = store.garbage_collect().await?;
            println!("Garbage collection complete. Deleted {} objects.", deleted_count);
        }
    }
    Ok(())
}
