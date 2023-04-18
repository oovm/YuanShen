use clap::Args;
use std::env::current_dir;
use ys_core::{
    initialize::{DotYuanShenClient, InitializeConfig},
    YsError,
};
use ys_core::initialize::YuanShenClient;

#[derive(Debug, Args)]
pub struct YuanShenCheckout {
    #[clap(short, long)]
    branch: String,
}

impl YuanShenCheckout {
    pub async fn checkout(self) -> Result<(), YsError> {
        let here = current_dir()?;
        let dot_rev = DotYuanShenClient::open(&here)?;
        dot_rev.set_branch(&self.branch)?;
        Ok(())
    }
}
