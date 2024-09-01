pub use clap::{command, Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub(crate) struct Args {
    #[command(subcommand)]
    pub(crate) command: Commands,
}

#[derive(Subcommand)]
pub(crate) enum Commands {
    Migrate,
    Daemon,
    Repos,
    MergeRequests,
}
