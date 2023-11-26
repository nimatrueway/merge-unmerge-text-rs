use std::path::PathBuf;

use clap::Parser;
use eyre::Result;
use log::{info, LevelFilter};
use simplelog::{ColorChoice, Config, TermLogger, TerminalMode};

use crate::args::{MergeArgs, UnmergeArgs};

mod args;
mod constants;
mod merger;
mod paths;

fn merge_fn(args: MergeArgs) -> Result<()> {
    let all_paths = paths::expand_and_merge(args.args)?;
    let merger = merger::MergerUnmerger::new(
        constants::PREPEND_DEFAULT.to_string(),
        constants::APPEND_DEFAULT.to_string(),
    )?;
    info!("{} files will be added to output", all_paths.len());

    merger.can_merge(&all_paths)?;
    merger.merge(&all_paths, &PathBuf::from(args.output.as_str()))?;

    Ok(())
}

fn unmerge_fn(args: UnmergeArgs) -> Result<()> {
    let merger = merger::MergerUnmerger::new(
        constants::PREPEND_DEFAULT.to_string(),
        constants::APPEND_DEFAULT.to_string(),
    )?;

    merger.unmerge(&PathBuf::from(args.input.as_str()))?;

    Ok(())
}

fn main() {
    // initialize logger
    TermLogger::init(
        LevelFilter::Info,
        Config::default(),
        TerminalMode::Stderr,
        ColorChoice::Auto,
    )
    .unwrap();

    // parse arguments
    let args = args::CliArgs::parse();

    match args.command {
        args::CliCommand::Merge(merge) => {
            merge_fn(merge).unwrap();
        }
        args::CliCommand::Unmerge(unmerge) => {
            unmerge_fn(unmerge).unwrap();
        }
    }
}
