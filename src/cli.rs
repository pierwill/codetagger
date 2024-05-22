use clap::{ArgAction, Parser};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// In order to make changes to the files,
    /// run `with --dryrun=false`.
    #[clap(long, short,
           default_missing_value("true"), default_value("true"), num_args(0..=1),
           require_equals(true), action = ArgAction::Set)]
    pub dryrun: bool,
    /// Path to the root of the target repo.
    #[arg(short, long)]
    pub repo: String,
    /// Print information on matches.
    #[arg(short, long)]
    pub verbose: bool,
}
