pub mod config;
pub mod get;

use clap::{Parser, Subcommand};
use clap_verbosity_flag::{self, Verbosity};
use config::Config;

use get::GetArgs;

#[derive(Parser, Debug)]
#[command(author, version)]
#[command(about = "vc4ctl - a CLI for interacting with Crestron VC-4 servers")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,    

    #[command(flatten)]
    pub verbose: Verbosity
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Configure server information
    Config(Config),
    Get(GetArgs)
}