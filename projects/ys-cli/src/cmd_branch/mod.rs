use clap::Args;
use std::env::current_dir;
use ys_core::{
    initialize::{DotYuanShenClient, InitializeConfig, YuanShenClient},
    YsError,
};

#[derive(Debug, Args)]
pub struct YuanShenBranch {

}

impl YuanShenBranch {
    pub async fn branch(self) -> Result<(), YsError> {
        let here = current_dir()?;
        let dot_rev = DotYuanShenClient::open(&here)?;
        let branch = dot_rev.get_branch()?;
        println!("{}", branch);
        Ok(())
    }
}
