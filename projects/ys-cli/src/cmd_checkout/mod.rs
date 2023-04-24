use clap::Args;
use std::env::current_dir;
use ys_core::{
    initialize::{DotYuanShenClient, YuanShenClient},
    YsError,
};

#[derive(Debug, Args)]
pub struct YuanShenCheckout {
    branch: String,
}

impl YuanShenCheckout {
    pub async fn checkout(self) -> Result<(), YsError> {
        let here = current_dir()?;
        let ys = DotYuanShenClient::open(&here)?;
        ys.create_branch(&self.branch)?;
        ys.set_branch(&self.branch)
    }
}
