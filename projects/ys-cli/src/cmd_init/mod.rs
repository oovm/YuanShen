use clap::Args;
use std::{borrow::Cow, env::current_dir};
use ys_core::{initialize::InitializeConfig, IgnoreRules, YsError};

#[derive(Debug, Args)]
pub struct YuanShenInitialize {
    /// override the name of the initial branch
    #[clap(long, short = 'b')]
    initial_branch: Option<String>,
}

impl YuanShenInitialize {
    pub async fn initialize(self) -> Result<(), YsError> {
        let config = InitializeConfig {
            current: current_dir()?,
            initial_branch: match self.initial_branch {
                Some(s) => Cow::Owned(s),
                None => Cow::Borrowed("master"),
            },
            ignores: IgnoreRules::default(),
        };
        config.generate().await?;
        Ok(())
    }
}
