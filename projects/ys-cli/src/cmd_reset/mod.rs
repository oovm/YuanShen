use clap::Args;


#[derive(Debug, Args)]
pub struct YuanShenReset {
    #[arg(long)]
    no_refresh: bool,
    #[arg(long)]
    mixed: bool,
    #[arg(long)]
    soft: bool,
    #[arg(long)]
    hard: bool,
    #[arg(long)]
    merge: bool,
    #[arg(long)]
    keep: bool,

}
