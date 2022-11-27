use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug)]
pub struct Config {
    #[command(subcommand)]
    pub config_commands: Option<ConfigCommands>,
}

#[derive(Subcommand, Debug)]
pub enum ConfigCommands {
    Add(ConfigAddArgs),
    Use(ConfigUseArgs),
    Remove(ConfigRemoveArgs),
    Update(ConfigUpdateArgs),
    GetServers(ConfigGetServersArgs),
}

#[derive(Args, Debug, Clone)]
pub struct ConfigAddArgs {
    pub name: String,
    pub url: String,
    pub token: String,
}

#[derive(Args, Debug, Clone)]
pub struct ConfigUseArgs {
    pub name: String,
}

#[derive(Args, Debug)]
pub struct ConfigRemoveArgs {
    pub name: String,
}

#[derive(Args, Debug)]
pub struct ConfigUpdateArgs {
    pub name: String,

    #[arg(short = 'u', long = "url")]
    pub url: Option<String>,

    #[arg(short = 't', long = "token")]
    pub token: Option<String>,
}

#[derive(Args, Debug)]
pub struct ConfigGetServersArgs {}
