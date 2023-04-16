#![feature(fs_try_exists)]

pub use crate::{
    cmd_checkout::YuanShenCheckout, cmd_commit::YuanShenCommit, cmd_diff::YuanShenDifference, cmd_init::YuanShenInitialize,
};
use clap::{Args, FromArgMatches, Parser, Subcommand};
use std::{env::current_dir, fmt::Debug, io::stdout};
use ys_core::{
    initialize::{DotYuanShen, InitializeConfig, InsertJson},
    Ignores, ObjectID, SnapShot, SnapShotDirectory, YsError,
};

mod cmd_checkout;
mod cmd_commit;
mod cmd_diff;
mod cmd_init;

#[derive(Parser, Debug)]
struct YuanShen {
    #[clap(subcommand)]
    cmd: YsCommand,
}

#[derive(Debug)]
enum YsCommand {
    /// 创建一个新的源神世界
    Initialize(YuanShenInitialize),
    /// 观测目标世界线与当前世界的差异
    Difference(YuanShenDifference),

    Changes,

    Commit(YuanShenCommit),
    /// 将观测结果合并到当前世界线
    Squash(YuanShenCommit),
    /// 设定世界线收束节点
    Merge(YuanShenCommit),
    /// 干涉目标世界线
    Rebase(YuanShenCommit),
    /// 回溯到任意固化节点
    Reverse(YuanShenCommit),
    /// 从某一条世界线开始开启一个新世界
    Orphan(YuanShenOrphan),
    /// 切换到指定名称的世界线
    Checkout(YuanShenCheckout),
    Branch,
    Stash(YuanShenCommit),
    /// 对象有点太城市化了
    GarbageCollect,

    External(Vec<String>),
}

#[automatically_derived]
impl FromArgMatches for YsCommand {
    fn from_arg_matches(__clap_arg_matches: &clap::ArgMatches) -> Result<Self, clap::Error> {
        Self::from_arg_matches_mut(&mut __clap_arg_matches.clone())
    }
    fn from_arg_matches_mut(__clap_arg_matches: &mut clap::ArgMatches) -> Result<Self, clap::Error> {
        #![allow(deprecated)]
        if let Some((__clap_name, mut __clap_arg_sub_matches)) = __clap_arg_matches.remove_subcommand() {
            let __clap_arg_matches = &mut __clap_arg_sub_matches;
            if __clap_name == "initialize" && !__clap_arg_matches.contains_id("") {
                return Ok(Self::Initialize(
                    YuanShenInitialize::from_arg_matches_mut(__clap_arg_matches)?,
                ));
            }
            if __clap_name == "difference" && !__clap_arg_matches.contains_id("") {
                return Ok(Self::Difference(
                    <YuanShenDifference as FromArgMatches>::from_arg_matches_mut(__clap_arg_matches)?,
                ));
            }
            if __clap_name == "changes" && !__clap_arg_matches.contains_id("") {
                return Ok(Self::Changes);
            }
            if __clap_name == "commit" && !__clap_arg_matches.contains_id("") {
                return Ok(Self::Commit(
                    <YuanShenCommit as FromArgMatches>::from_arg_matches_mut(__clap_arg_matches)?,
                ));
            }
            if __clap_name == "squash" && !__clap_arg_matches.contains_id("") {
                return Ok(Self::Squash(
                    <YuanShenCommit as FromArgMatches>::from_arg_matches_mut(__clap_arg_matches)?,
                ));
            }
            if __clap_name == "merge" && !__clap_arg_matches.contains_id("") {
                return Ok(Self::Merge(<YuanShenCommit as FromArgMatches>::from_arg_matches_mut(
                    __clap_arg_matches,
                )?));
            }
            if __clap_name == "rebase" && !__clap_arg_matches.contains_id("") {
                return Ok(Self::Rebase(
                    <YuanShenCommit as FromArgMatches>::from_arg_matches_mut(__clap_arg_matches)?,
                ));
            }
            if __clap_name == "reverse" && !__clap_arg_matches.contains_id("") {
                return Ok(Self::Reverse(
                    <YuanShenCommit as FromArgMatches>::from_arg_matches_mut(__clap_arg_matches)?,
                ));
            }
            if __clap_name == "orphan" && !__clap_arg_matches.contains_id("") {
                return Ok(Self::Orphan(
                    <YuanShenOrphan as FromArgMatches>::from_arg_matches_mut(__clap_arg_matches)?,
                ));
            }
            if __clap_name == "checkout" && !__clap_arg_matches.contains_id("") {
                return Ok(Self::Checkout(
                    <YuanShenCheckout as FromArgMatches>::from_arg_matches_mut(__clap_arg_matches)?,
                ));
            }
            if __clap_name == "branch" && !__clap_arg_matches.contains_id("") {
                return Ok(Self::Branch);
            }
            if __clap_name == "stash" && !__clap_arg_matches.contains_id("") {
                return Ok(Self::Stash(<YuanShenCommit as FromArgMatches>::from_arg_matches_mut(
                    __clap_arg_matches,
                )?));
            }
            if __clap_name == "garbage-collect" && !__clap_arg_matches.contains_id("") {
                return Ok(Self::GarbageCollect);
            }
            Ok(Self::External(
                std::iter::once(::std::string::String::from(__clap_name))
                    .chain(
                        __clap_arg_matches.remove_many::<String>("").unwrap().map(::std::string::String::from),
                    )
                    .collect::<Vec<_>>(),
            ))
        }
        else {
            Err(clap::Error::raw(
                clap::error::ErrorKind::MissingSubcommand,
                "A subcommand is required but one was not provided.",
            ))
        }
    }
    fn update_from_arg_matches(&mut self, __clap_arg_matches: &clap::ArgMatches) -> Result<(), clap::Error> {
        self.update_from_arg_matches_mut(&mut __clap_arg_matches.clone())
    }
    fn update_from_arg_matches_mut<'b>(
        &mut self,
        __clap_arg_matches: &mut clap::ArgMatches,
    ) -> Result<(), clap::Error> {
        #![allow(deprecated)]
        if let Some(__clap_name) = __clap_arg_matches.subcommand_name() {
            match self {
                Self::Initialize(ref mut __clap_arg) if "initialize" == __clap_name => {
                    let (_, mut __clap_arg_sub_matches) = __clap_arg_matches.remove_subcommand().unwrap();
                    let __clap_arg_matches = &mut __clap_arg_sub_matches;
                    FromArgMatches::update_from_arg_matches_mut(__clap_arg, __clap_arg_matches)?
                }
                Self::Difference(ref mut __clap_arg) if "difference" == __clap_name => {
                    let (_, mut __clap_arg_sub_matches) = __clap_arg_matches.remove_subcommand().unwrap();
                    let __clap_arg_matches = &mut __clap_arg_sub_matches;
                    FromArgMatches::update_from_arg_matches_mut(__clap_arg, __clap_arg_matches)?
                }
                Self::Changes if "changes" == __clap_name => {
                    let (_, mut __clap_arg_sub_matches) = __clap_arg_matches.remove_subcommand().unwrap();
                    let __clap_arg_matches = &mut __clap_arg_sub_matches;
                    {}
                }
                Self::Commit(ref mut __clap_arg) if "commit" == __clap_name => {
                    let (_, mut __clap_arg_sub_matches) = __clap_arg_matches.remove_subcommand().unwrap();
                    let __clap_arg_matches = &mut __clap_arg_sub_matches;
                    FromArgMatches::update_from_arg_matches_mut(__clap_arg, __clap_arg_matches)?
                }
                Self::Squash(ref mut __clap_arg) if "squash" == __clap_name => {
                    let (_, mut __clap_arg_sub_matches) = __clap_arg_matches.remove_subcommand().unwrap();
                    let __clap_arg_matches = &mut __clap_arg_sub_matches;
                    FromArgMatches::update_from_arg_matches_mut(__clap_arg, __clap_arg_matches)?
                }
                Self::Merge(ref mut __clap_arg) if "merge" == __clap_name => {
                    let (_, mut __clap_arg_sub_matches) = __clap_arg_matches.remove_subcommand().unwrap();
                    let __clap_arg_matches = &mut __clap_arg_sub_matches;
                    FromArgMatches::update_from_arg_matches_mut(__clap_arg, __clap_arg_matches)?
                }
                Self::Rebase(ref mut __clap_arg) if "rebase" == __clap_name => {
                    let (_, mut __clap_arg_sub_matches) = __clap_arg_matches.remove_subcommand().unwrap();
                    let __clap_arg_matches = &mut __clap_arg_sub_matches;
                    FromArgMatches::update_from_arg_matches_mut(__clap_arg, __clap_arg_matches)?
                }
                Self::Reverse(ref mut __clap_arg) if "reverse" == __clap_name => {
                    let (_, mut __clap_arg_sub_matches) = __clap_arg_matches.remove_subcommand().unwrap();
                    let __clap_arg_matches = &mut __clap_arg_sub_matches;
                    FromArgMatches::update_from_arg_matches_mut(__clap_arg, __clap_arg_matches)?
                }
                Self::Orphan(ref mut __clap_arg) if "orphan" == __clap_name => {
                    let (_, mut __clap_arg_sub_matches) = __clap_arg_matches.remove_subcommand().unwrap();
                    let __clap_arg_matches = &mut __clap_arg_sub_matches;
                    FromArgMatches::update_from_arg_matches_mut(__clap_arg, __clap_arg_matches)?
                }
                Self::Checkout(ref mut __clap_arg) if "checkout" == __clap_name => {
                    let (_, mut __clap_arg_sub_matches) = __clap_arg_matches.remove_subcommand().unwrap();
                    let __clap_arg_matches = &mut __clap_arg_sub_matches;
                    FromArgMatches::update_from_arg_matches_mut(__clap_arg, __clap_arg_matches)?
                }
                Self::Branch if "branch" == __clap_name => {
                    let (_, mut __clap_arg_sub_matches) = __clap_arg_matches.remove_subcommand().unwrap();
                    let __clap_arg_matches = &mut __clap_arg_sub_matches;
                    {}
                }
                Self::Stash(ref mut __clap_arg) if "stash" == __clap_name => {
                    let (_, mut __clap_arg_sub_matches) = __clap_arg_matches.remove_subcommand().unwrap();
                    let __clap_arg_matches = &mut __clap_arg_sub_matches;
                    FromArgMatches::update_from_arg_matches_mut(__clap_arg, __clap_arg_matches)?
                }
                Self::GarbageCollect if "garbage-collect" == __clap_name => {
                    let (_, mut __clap_arg_sub_matches) = __clap_arg_matches.remove_subcommand().unwrap();
                    let __clap_arg_matches = &mut __clap_arg_sub_matches;
                    {}
                }
                s => {
                    *s = <Self as FromArgMatches>::from_arg_matches_mut(__clap_arg_matches)?;
                }
            }
        }
        Ok(())
    }
}
#[allow(dead_code, unreachable_code, unused_variables, unused_braces, unused_qualifications)]
#[allow(
    clippy::style,
    clippy::complexity,
    clippy::pedantic,
    clippy::restriction,
    clippy::perf,
    clippy::deprecated,
    clippy::nursery,
    clippy::cargo,
    clippy::suspicious_else_formatting,
    clippy::almost_swapped,
    clippy::redundant_locals
)]
#[automatically_derived]
impl clap::Subcommand for YsCommand {
    fn augment_subcommands<'b>(__clap_app: clap::Command) -> clap::Command {
        let __clap_app = __clap_app;
        let __clap_app = __clap_app.subcommand({
            let __clap_subcommand = clap::Command::new("initialize");
            let __clap_subcommand = __clap_subcommand;
            let __clap_subcommand = { <YuanShenInitialize as clap::Args>::augment_args(__clap_subcommand) };
            __clap_subcommand.about("创建一个新的源神世界").long_about(None).alias("init").alias("启动!")
        });
        let __clap_app = __clap_app.subcommand({
            let __clap_subcommand = clap::Command::new("difference");
            let __clap_subcommand = __clap_subcommand;
            let __clap_subcommand = { <YuanShenDifference as clap::Args>::augment_args(__clap_subcommand) };
            __clap_subcommand.about("观测目标世界线与当前世界的差异").long_about(None).alias("观测")
        });
        let __clap_app = __clap_app.subcommand({
            let __clap_subcommand = clap::Command::new("changes");
            let __clap_subcommand = __clap_subcommand;
            let __clap_subcommand = __clap_subcommand;
            __clap_subcommand.alias("异变")
        });
        let __clap_app = __clap_app.subcommand({
            let __clap_subcommand = clap::Command::new("commit");
            let __clap_subcommand = __clap_subcommand;
            let __clap_subcommand = { <YuanShenCommit as clap::Args>::augment_args(__clap_subcommand) };
            __clap_subcommand.alias("衍化")
        });
        let __clap_app = __clap_app.subcommand({
            let __clap_subcommand = clap::Command::new("squash");
            let __clap_subcommand = __clap_subcommand;
            let __clap_subcommand = { <YuanShenCommit as clap::Args>::augment_args(__clap_subcommand) };
            __clap_subcommand.about("将观测结果合并到当前世界线").long_about(None).alias("塌缩")
        });
        let __clap_app = __clap_app.subcommand({
            let __clap_subcommand = clap::Command::new("merge");
            let __clap_subcommand = __clap_subcommand;
            let __clap_subcommand = { <YuanShenCommit as clap::Args>::augment_args(__clap_subcommand) };
            __clap_subcommand.about("设定世界线收束节点").long_about(None).alias("收束")
        });
        let __clap_app = __clap_app.subcommand({
            let __clap_subcommand = clap::Command::new("rebase");
            let __clap_subcommand = __clap_subcommand;
            let __clap_subcommand = { <YuanShenCommit as clap::Args>::augment_args(__clap_subcommand) };
            __clap_subcommand.about("干涉目标世界线").long_about(None).alias("干涉")
        });
        let __clap_app = __clap_app.subcommand({
            let __clap_subcommand = clap::Command::new("reverse");
            let __clap_subcommand = __clap_subcommand;
            let __clap_subcommand = { <YuanShenCommit as clap::Args>::augment_args(__clap_subcommand) };
            __clap_subcommand.about("回溯到任意固化节点").long_about(None).alias("回溯")
        });
        let __clap_app = __clap_app.subcommand({
            let __clap_subcommand = clap::Command::new("orphan");
            let __clap_subcommand = __clap_subcommand;
            let __clap_subcommand = { <YuanShenOrphan as clap::Args>::augment_args(__clap_subcommand) };
            __clap_subcommand.about("从某一条世界线开始开启一个新世界").long_about(None).alias("退相干")
        });
        let __clap_app = __clap_app.subcommand({
            let __clap_subcommand = clap::Command::new("checkout");
            let __clap_subcommand = __clap_subcommand;
            let __clap_subcommand = { <YuanShenCheckout as clap::Args>::augment_args(__clap_subcommand) };
            __clap_subcommand.about("切换到指定名称的世界线").long_about(None).alias("跃迁")
        });
        let __clap_app = __clap_app.subcommand({
            let __clap_subcommand = clap::Command::new("branch");
            let __clap_subcommand = __clap_subcommand;
            let __clap_subcommand = __clap_subcommand;
            __clap_subcommand
        });
        let __clap_app = __clap_app.subcommand({
            let __clap_subcommand = clap::Command::new("stash");
            let __clap_subcommand = __clap_subcommand;
            let __clap_subcommand = { <YuanShenCommit as clap::Args>::augment_args(__clap_subcommand) };
            __clap_subcommand
        });
        let __clap_app = __clap_app.subcommand({
            let __clap_subcommand = clap::Command::new("garbage-collect");
            let __clap_subcommand = __clap_subcommand;
            let __clap_subcommand = __clap_subcommand;
            __clap_subcommand.about("对象有点太城市化了").long_about(None).alias("gc").alias("逆城市化")
        });
        let __clap_app = __clap_app.external_subcommand_value_parser({
            use clap_builder::builder::via_prelude::*;
            let auto = clap_builder::builder::_AutoValueParser::<String>::new();
            (&&&&&&auto).value_parser()
        });
        __clap_app
    }
    fn augment_subcommands_for_update<'b>(__clap_app: clap::Command) -> clap::Command {
        let __clap_app = __clap_app;
        let __clap_app = __clap_app.subcommand({
            let __clap_subcommand = clap::Command::new("initialize");
            let __clap_subcommand = __clap_subcommand;
            let __clap_subcommand = { <YuanShenInitialize as clap::Args>::augment_args_for_update(__clap_subcommand) };
            __clap_subcommand.about("创建一个新的源神世界").long_about(None).alias("init").alias("启动!")
        });
        let __clap_app = __clap_app.subcommand({
            let __clap_subcommand = clap::Command::new("difference");
            let __clap_subcommand = __clap_subcommand;
            let __clap_subcommand = { <YuanShenDifference as clap::Args>::augment_args_for_update(__clap_subcommand) };
            __clap_subcommand.about("观测目标世界线与当前世界的差异").long_about(None).alias("观测")
        });
        let __clap_app = __clap_app.subcommand({
            let __clap_subcommand = clap::Command::new("changes");
            let __clap_subcommand = __clap_subcommand;
            let __clap_subcommand = __clap_subcommand;
            __clap_subcommand.alias("异变")
        });
        let __clap_app = __clap_app.subcommand({
            let __clap_subcommand = clap::Command::new("commit");
            let __clap_subcommand = __clap_subcommand;
            let __clap_subcommand = { <YuanShenCommit as clap::Args>::augment_args_for_update(__clap_subcommand) };
            __clap_subcommand.alias("衍化")
        });
        let __clap_app = __clap_app.subcommand({
            let __clap_subcommand = clap::Command::new("squash");
            let __clap_subcommand = __clap_subcommand;
            let __clap_subcommand = { <YuanShenCommit as clap::Args>::augment_args_for_update(__clap_subcommand) };
            __clap_subcommand.about("将观测结果合并到当前世界线").long_about(None).alias("塌缩")
        });
        let __clap_app = __clap_app.subcommand({
            let __clap_subcommand = clap::Command::new("merge");
            let __clap_subcommand = __clap_subcommand;
            let __clap_subcommand = { <YuanShenCommit as clap::Args>::augment_args_for_update(__clap_subcommand) };
            __clap_subcommand.about("设定世界线收束节点").long_about(None).alias("收束")
        });
        let __clap_app = __clap_app.subcommand({
            let __clap_subcommand = clap::Command::new("rebase");
            let __clap_subcommand = __clap_subcommand;
            let __clap_subcommand = { <YuanShenCommit as clap::Args>::augment_args_for_update(__clap_subcommand) };
            __clap_subcommand.about("干涉目标世界线").long_about(None).alias("干涉")
        });
        let __clap_app = __clap_app.subcommand({
            let __clap_subcommand = clap::Command::new("reverse");
            let __clap_subcommand = __clap_subcommand;
            let __clap_subcommand = { <YuanShenCommit as clap::Args>::augment_args_for_update(__clap_subcommand) };
            __clap_subcommand.about("回溯到任意固化节点").long_about(None).alias("回溯")
        });
        let __clap_app = __clap_app.subcommand({
            let __clap_subcommand = clap::Command::new("orphan");
            let __clap_subcommand = __clap_subcommand;
            let __clap_subcommand = { <YuanShenOrphan as clap::Args>::augment_args_for_update(__clap_subcommand) };
            __clap_subcommand.about("从某一条世界线开始开启一个新世界").long_about(None).alias("退相干")
        });
        let __clap_app = __clap_app.subcommand({
            let __clap_subcommand = clap::Command::new("checkout");
            let __clap_subcommand = __clap_subcommand;
            let __clap_subcommand = { <YuanShenCheckout as clap::Args>::augment_args_for_update(__clap_subcommand) };
            __clap_subcommand.about("切换到指定名称的世界线").long_about(None).alias("跃迁")
        });
        let __clap_app = __clap_app.subcommand({
            let __clap_subcommand = clap::Command::new("branch");
            let __clap_subcommand = __clap_subcommand;
            let __clap_subcommand = __clap_subcommand;
            __clap_subcommand
        });
        let __clap_app = __clap_app.subcommand({
            let __clap_subcommand = clap::Command::new("stash");
            let __clap_subcommand = __clap_subcommand;
            let __clap_subcommand = { <YuanShenCommit as clap::Args>::augment_args_for_update(__clap_subcommand) };
            __clap_subcommand
        });
        let __clap_app = __clap_app.subcommand({
            let __clap_subcommand = clap::Command::new("garbage-collect");
            let __clap_subcommand = __clap_subcommand;
            let __clap_subcommand = __clap_subcommand;
            __clap_subcommand.about("对象有点太城市化了").long_about(None).alias("gc").alias("逆城市化")
        });
        let __clap_app = __clap_app.external_subcommand_value_parser({
            use ::clap_builder::builder::via_prelude::*;
            let auto = ::clap_builder::builder::_AutoValueParser::<String>::new();
            (&&&&&&auto).value_parser()
        });
        __clap_app
    }
    fn has_subcommand(__clap_name: &str) -> bool {
        true
    }
}

#[derive(Debug, Args)]
pub struct YuanShenOrphan {}

#[tokio::main]
pub async fn main() -> Result<(), YsError> {
    let args = YuanShen::parse();
    use YsCommand::*;
    match args.cmd {
        Initialize(init) => init.initialize().await?,
        Difference(diff) => diff.difference().await?,
        Branch => {
            let dot_rev = DotYuanShen::open(current_dir().unwrap().join(".ys")).unwrap();
            let branch = dot_rev.get_branch().unwrap();
            println!("{}", branch);
        }
        Checkout(c) => c.checkout().await?,
        Changes => {
            let dir = current_dir().unwrap();
            let rev_dir = dir.join(".ys");
            let dot_rev = DotYuanShen::open(rev_dir).unwrap();
            let mut store = dot_rev.store().unwrap();
            let branch: String = dot_rev.get_branch().unwrap();
            let old_tip: ObjectID = dot_rev.get_branch_snapshot_id(&branch).unwrap();
            let ignores: Ignores = dot_rev.ignores().unwrap();
            let directory = SnapShotDirectory::new(dir.as_path(), &ignores, &mut store).unwrap();
            let snapshot: SnapShot = store.read_json(old_tip).await.unwrap();
            let old_directory: SnapShotDirectory = store.read_json(snapshot.directory).await.unwrap();
            serde_json::to_writer_pretty(stdout(), &old_directory.difference(&directory)).unwrap();
        }
        Commit(sub) => sub.commit().await.unwrap(),
        Squash(_) => {}
        Merge(_) => {}
        Rebase(_) => {}
        Reverse(_) => {}
        Orphan(_) => {}
        Stash(_) => {}
        External(_) => {}
        GarbageCollect => {}
    }
    Ok(())
}
