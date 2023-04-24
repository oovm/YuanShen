use clap::Args;
use std::{env::current_dir, process::exit};
use ys_core::{
    initialize::{DotYuanShenClient, YuanShenClient},
    ObjectID, ObjectStore, SnapShot, SnapShotDirectory, YsError,
};

#[derive(Debug, Args)]
pub struct YuanShenDifference {
    branch: String,
}

impl YuanShenDifference {
    pub async fn difference(self) -> Result<(), YsError> {
        let dir = current_dir().unwrap();
        let dot_rev = DotYuanShenClient::open(&dir)?;
        let store = dot_rev.store()?;
        let that_branch = self.branch.as_ref();
        let this_branch: String = dot_rev.get_branch_name()?;
        if !dot_rev.branch_exists(&that_branch)? {
            eprintln!("no branch named {} exists", that_branch);
            exit(1);
        }
        let this_tip: ObjectID = dot_rev.get_branch_id(&this_branch)?;
        let that_tip: ObjectID = dot_rev.get_branch_id(&that_branch)?;
        let that_snapshot: SnapShot = store.get_typed(that_tip).await.expect("read that tip");
        let that_branch_directory = store.get_typed(that_snapshot.directory).await.expect("read that directory");
        let this_snapshot: SnapShot = store.get_typed(this_tip).await.expect("read this tip");
        let this_branch_directory: SnapShotDirectory =
            store.get_typed(this_snapshot.directory).await.expect("read this branch directory");
        let diff = &this_branch_directory.difference(&that_branch_directory);
        println!("{diff}");
        Ok(())
    }
}
