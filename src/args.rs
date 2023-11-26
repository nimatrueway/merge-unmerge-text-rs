use clap::*;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct CliArgs {
    #[command(subcommand)]
    pub command: CliCommand,
}

#[derive(Subcommand, Debug)]
pub enum CliCommand {
    Merge(MergeArgs),
    Unmerge(UnmergeArgs),
}

#[derive(Parser, Debug)]
pub struct MergeArgs {
    #[arg(short, long)]
    /// path to save the merged file, list of files to merge needs to be piped to m
    pub output: String,
    pub args: Vec<String>,
}

#[derive(Parser, Debug)]
pub struct UnmergeArgs {
    /// path to the merged file to unmerge in the current directory
    pub input: String,
}
