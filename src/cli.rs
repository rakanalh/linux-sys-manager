use std::path::PathBuf;

use clap::{ArgAction, Args, Parser, Subcommand};

#[derive(Debug, Args)]
struct GlobalOpts {
    #[clap(long, short, global = true, action = ArgAction::Count)]
    verbose: u8,
}

#[derive(Debug, Parser)]
#[clap(name = "LSM", version)]
pub struct App {
    #[clap(flatten)]
    global_opts: GlobalOpts,

    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Help message for read.
    Sync {
        /// The path to read from
        #[clap(long, short = 'c')]
        config_path: PathBuf,
        #[clap(long, short = 's')]
        system: bool,
        #[clap(long, short = 'm')]
        home: bool,
    },
}
