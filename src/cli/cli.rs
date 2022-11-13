use clap::{Parser, Subcommand, Args};

#[derive(Parser)]
#[command(author, version)]
#[command(about="vc4ctl - a CLI for interacting with Crestron VC-4 servers")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Configure server information
    Config(Config),
}

#[derive(Parser)]
pub struct Config {
  #[command(subcommand)]
  pub config_commands:Option<ConfigCommands>,
}

#[derive(Subcommand)]
pub enum ConfigCommands {
  Add(ConfigAddArgs),  
  Use(ConfigUseArgs),
  Remove(ConfigRemoveArgs),
  Update(ConfigUpdateArgs),
  GetServers(ConfigGetServersArgs),
}

#[derive(Args)]
pub struct ConfigAddArgs {
  pub name: String,
  pub url: String,  
  pub token: String,
}

#[derive(Args)]
pub struct ConfigUseArgs {
  pub name: String,
}

#[derive(Args)]
pub struct ConfigRemoveArgs {
  pub name: String,
}

#[derive(Args)]
pub struct ConfigUpdateArgs {
  pub name: String,
  pub url: String,
  pub token: String,
}

#[derive(Args)]
pub struct ConfigGetServersArgs {
  
}