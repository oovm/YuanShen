use std::env::current_dir;
use std::process::exit;
use clap::Args;
use ys_core::{DotYuanShen, ObjectID, SnapShot, SnapShotDirectory, YsError};

#[derive(Debug, Args)]
pub struct YuanShenDifference {
    branch: String,
}

impl YuanShenDifference {
    pub async fn run(&self) -> Result<(), YsError> {
        let dir = current_dir().unwrap();
        let rev_dir = dir.join(".ys");
        let dot_rev = DotYuanShen::open(rev_dir).unwrap();
        let mut store = dot_rev.store().unwrap();
        let that_branch = self.branch.as_ref();
        let this_branch: String = dot_rev.get_branch().unwrap();
        if !dot_rev.branch_exists(&that_branch).unwrap() {
            eprintln!("no branch named {} exists", that_branch);
            exit(1);
        }
        let this_tip: ObjectID = dot_rev.get_branch_snapshot_id(&this_branch).unwrap();
        let that_tip: ObjectID = dot_rev.get_branch_snapshot_id(&that_branch).unwrap();
        let that_snapshot: SnapShot = store.read_json(that_tip).await.expect("read that tip");
        let that_branch_directory = store.read_json(that_snapshot.directory).await.expect("read that directory");
        let this_snapshot: SnapShot = store.read_json(this_tip).await.expect("read this tip");
        let this_branch_directory: SnapShotDirectory =
            store.read_json(this_snapshot.directory).await.expect("read this branch directory");
        let diff = &this_branch_directory.difference(&that_branch_directory);
        println!("{diff}");
        Ok(())
    }
}
