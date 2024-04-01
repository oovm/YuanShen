use clap::{ArgMatches, Args, FromArgMatches, Parser, Subcommand};
use clap_builder::{
    builder::{_AutoValueParser, via_prelude::_ValueParserViaParse},
    Command,
};
use std::{env::current_dir, fmt::Debug, io::stdout};
use ys_core::{
    initialize::{DotYuanShenClient, YuanShenClient},
    IgnoreRules, ObjectID, ObjectStore, SnapShot, SnapShotDirectory, YsError,
};
use yuan_shen::*;

#[derive(Parser, Debug)]
struct YuanShen {
    #[clap(subcommand)]
    cmd: YsCommand,
}

#[derive(Debug)]
enum YsCommand {
    Initialize(YuanShenInitialize),
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
    Stash(YuanShenCommit),
    GarbageCollect,
    External(Vec<String>),
}

#[automatically_derived]
impl FromArgMatches for YsCommand {
    fn from_arg_matches(args: &ArgMatches) -> Result<Self, clap::Error> {
        Self::from_arg_matches_mut(&mut args.clone())
    }
    fn from_arg_matches_mut(args: &mut ArgMatches) -> Result<Self, clap::Error> {
        match args.remove_subcommand() {
            Some((cmd, ref mut sub_args)) => {
                let ys_cmd = match cmd.as_str() {
                    "initialize" | "启动!" | "启动" => Self::Initialize(FromArgMatches::from_arg_matches_mut(sub_args)?),
                    "difference" | "观测" => Self::Difference(FromArgMatches::from_arg_matches_mut(sub_args)?),
                    "orphan" | "退相干" => Self::Orphan(FromArgMatches::from_arg_matches_mut(sub_args)?),
                    "squash" | "塌缩" => Self::Squash(FromArgMatches::from_arg_matches_mut(sub_args)?),
                    "merge" | "收束" => Self::Merge(FromArgMatches::from_arg_matches_mut(sub_args)?),
                    "rebase" | "干涉" => Self::Rebase(FromArgMatches::from_arg_matches_mut(sub_args)?),
                    "reverse" | "回溯" => Self::Reset(FromArgMatches::from_arg_matches_mut(sub_args)?),
                    "checkout" | "跃迁" => Self::Checkout(FromArgMatches::from_arg_matches_mut(sub_args)?),
                    "branch" => Self::Branch(FromArgMatches::from_arg_matches_mut(sub_args)?),
                    "stash" => Self::Stash(FromArgMatches::from_arg_matches_mut(sub_args)?),
                    "commit" => Self::Commit(FromArgMatches::from_arg_matches_mut(sub_args)?),
                    "changes" => Self::Changes,
                    "garbage-collect" => Self::GarbageCollect,
                    _ => Self::External(
                        std::iter::once(String::from(cmd))
                            .chain(sub_args.remove_many::<String>("").unwrap().map(String::from))
                            .collect::<Vec<_>>(),
                    ),
                };
                return Ok(ys_cmd);
            }
            None => Err(clap::Error::raw(
                clap::error::ErrorKind::MissingSubcommand,
                "A subcommand is required but one was not provided.",
            )),
        }
    }
    fn update_from_arg_matches(&mut self, args: &ArgMatches) -> Result<(), clap::Error> {
        self.update_from_arg_matches_mut(&mut args.clone())
    }
    fn update_from_arg_matches_mut<'b>(&mut self, args: &mut ArgMatches) -> Result<(), clap::Error> {
        match args.subcommand_name() {
            Some(clap) => match self {
                Self::Initialize(ref mut __clap_arg) if "initialize" == clap => {
                    let (_, mut __clap_arg_sub_matches) = args.remove_subcommand().unwrap();
                    let __clap_arg_matches = &mut __clap_arg_sub_matches;
                    FromArgMatches::update_from_arg_matches_mut(__clap_arg, __clap_arg_matches)?
                }
                Self::Difference(ref mut __clap_arg) if "difference" == clap => {
                    let (_, mut __clap_arg_sub_matches) = args.remove_subcommand().unwrap();
                    let __clap_arg_matches = &mut __clap_arg_sub_matches;
                    FromArgMatches::update_from_arg_matches_mut(__clap_arg, __clap_arg_matches)?
                }
                Self::Changes if "changes" == clap => {
                    let (_, mut __clap_arg_sub_matches) = args.remove_subcommand().unwrap();
                    let __clap_arg_matches = &mut __clap_arg_sub_matches;
                    {}
                }
                Self::Commit(ref mut __clap_arg) if "commit" == clap => {
                    let (_, mut __clap_arg_sub_matches) = args.remove_subcommand().unwrap();
                    let __clap_arg_matches = &mut __clap_arg_sub_matches;
                    FromArgMatches::update_from_arg_matches_mut(__clap_arg, __clap_arg_matches)?
                }
                Self::Squash(ref mut __clap_arg) if "squash" == clap => {
                    let (_, mut __clap_arg_sub_matches) = args.remove_subcommand().unwrap();
                    let __clap_arg_matches = &mut __clap_arg_sub_matches;
                    FromArgMatches::update_from_arg_matches_mut(__clap_arg, __clap_arg_matches)?
                }
                Self::Merge(ref mut __clap_arg) if "merge" == clap => {
                    let (_, mut __clap_arg_sub_matches) = args.remove_subcommand().unwrap();
                    let __clap_arg_matches = &mut __clap_arg_sub_matches;
                    FromArgMatches::update_from_arg_matches_mut(__clap_arg, __clap_arg_matches)?
                }
                Self::Rebase(ref mut __clap_arg) if "rebase" == clap => {
                    let (_, mut __clap_arg_sub_matches) = args.remove_subcommand().unwrap();
                    let __clap_arg_matches = &mut __clap_arg_sub_matches;
                    FromArgMatches::update_from_arg_matches_mut(__clap_arg, __clap_arg_matches)?
                }
                Self::Reset(ref mut __clap_arg) if "reverse" == clap => {
                    let (_, mut __clap_arg_sub_matches) = args.remove_subcommand().unwrap();
                    let __clap_arg_matches = &mut __clap_arg_sub_matches;
                    FromArgMatches::update_from_arg_matches_mut(__clap_arg, __clap_arg_matches)?
                }
                Self::Orphan(ref mut __clap_arg) if "orphan" == clap => {
                    let (_, mut __clap_arg_sub_matches) = args.remove_subcommand().unwrap();
                    let __clap_arg_matches = &mut __clap_arg_sub_matches;
                    FromArgMatches::update_from_arg_matches_mut(__clap_arg, __clap_arg_matches)?
                }
                Self::Checkout(ref mut __clap_arg) if "checkout" == clap => {
                    let (_, mut __clap_arg_sub_matches) = args.remove_subcommand().unwrap();
                    let __clap_arg_matches = &mut __clap_arg_sub_matches;
                    FromArgMatches::update_from_arg_matches_mut(__clap_arg, __clap_arg_matches)?
                }
                Self::Branch(ref mut __clap_arg) if "branch" == clap => {
                    let (_, mut __clap_arg_sub_matches) = args.remove_subcommand().unwrap();
                    let __clap_arg_matches = &mut __clap_arg_sub_matches;
                    FromArgMatches::update_from_arg_matches_mut(__clap_arg, __clap_arg_matches)?
                }
                Self::Stash(ref mut __clap_arg) if "stash" == clap => {
                    let (_, mut __clap_arg_sub_matches) = args.remove_subcommand().unwrap();
                    let __clap_arg_matches = &mut __clap_arg_sub_matches;
                    FromArgMatches::update_from_arg_matches_mut(__clap_arg, __clap_arg_matches)?
                }
                Self::GarbageCollect if "garbage-collect" == clap => {
                    let (_, mut __clap_arg_sub_matches) = args.remove_subcommand().unwrap();
                    let __clap_arg_matches = &mut __clap_arg_sub_matches;
                    {}
                }
                s => {
                    *s = <Self as FromArgMatches>::from_arg_matches_mut(args)?;
                }
            },
            None => {}
        }
        Ok(())
    }
}

impl Subcommand for YsCommand {
    fn augment_subcommands<'b>(app: Command) -> Command {
        app.subcommand(
            YuanShenInitialize::augment_args(Command::new("启动!"))
                .about("创建一个新的源神世界")
                .long_about(None)
                .alias("initialize")
                .alias("init")
                .display_name("启动!"),
        )
        .subcommand(
            YuanShenDifference::augment_args(Command::new("观测"))
                .about("观测目标世界线与当前世界的差异")
                .long_about(None)
                .alias("difference"),
        )
        .subcommand(Command::new("异变").alias("changes"))
        .subcommand(YuanShenCommit::augment_args(Command::new("衍化")).alias("commit"))
        .subcommand(
            YuanShenCommit::augment_args(Command::new("塌缩"))
                .about("将观测结果合并到当前世界线")
                .long_about(None)
                .alias("squash"),
        )
        .subcommand(
            YuanShenMerge::augment_args(Command::new("收束")).about("设定世界线收束节点").long_about(None).alias("merge"),
        )
        .subcommand(YuanShenCommit::augment_args(Command::new("干涉")).about("干涉目标世界线").long_about(None).alias("rebase"))
        .subcommand(
            YuanShenCommit::augment_args(Command::new("回溯")).about("回溯到任意固化节点").long_about(None).alias("reverse"),
        )
        .subcommand(
            YuanShenOrphan::augment_args(Command::new("退相干"))
                .about("从某一条世界线开始开启一个新世界")
                .long_about(None)
                .alias("orphan"),
        )
        .subcommand({
            YuanShenCheckout::augment_args(Command::new("跃迁"))
                .about("切换到指定名称的世界线")
                .long_about(None)
                .alias("checkout")
        })
        .subcommand(Command::new("branch"))
        .subcommand(YuanShenCommit::augment_args(Command::new("stash")))
        .subcommand(Command::new("逆化").about("这些对象有点太城市化了").long_about(None).alias("gc").alias("garbage-collect"))
        .external_subcommand_value_parser(_AutoValueParser::<String>::new().value_parser())
    }

    fn augment_subcommands_for_update(cmd: Command) -> Command {
        Self::augment_subcommands(cmd)
    }

    fn has_subcommand(_: &str) -> bool {
        true
    }
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
            let mut store = dot_rev.store().unwrap();
            let branch: String = dot_rev.get_branch_name().unwrap();
            let old_tip: ObjectID = dot_rev.get_branch_id(&branch).unwrap();
            let ignores: IgnoreRules = dot_rev.ignores().unwrap();
            let directory = SnapShotDirectory::new(dir.as_path(), &ignores, &mut store).unwrap();
            let snapshot: SnapShot = store.get_typed(old_tip).await.unwrap();
            let old_directory: SnapShotDirectory = store.get_typed(snapshot.directory).await.unwrap();
            serde_json::to_writer_pretty(stdout(), &old_directory.difference(&directory)).unwrap();
        }
        Commit(sub) => sub.commit().await.unwrap(),
        Squash(_) => {}
        Merge(_) => {}
        Rebase(_) => {}
        Reset(_) => {}
        Orphan(_) => {}
        Stash(_) => {}
        External(_) => {}
        GarbageCollect => {}
    }
    Ok(())
}
