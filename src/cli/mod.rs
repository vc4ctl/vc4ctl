pub mod config;
pub mod get;
pub mod room_control;

use clap::{Parser, Subcommand};
use clap_verbosity_flag::{self, Verbosity};
use config::Config;

use get::GetArgs;
use room_control::RoomControlArgs;

#[derive(Parser, Debug)]
#[command(author, version)]
#[command(about = "vc4ctl - a CLI for interacting with Crestron VC-4 servers")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,

    #[command(flatten)]
    pub verbose: Verbosity,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Configure server information
    Config(Config),
    /// Get information about a resource (room, program, etc.)
    Get(GetArgs),
    /// Stop a room
    Stop(RoomControlArgs),
    /// Start a room
    Start(RoomControlArgs),
}
