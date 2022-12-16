use clap::{Args, Parser, Subcommand};

use crate::api::config as api_config;

#[derive(Parser, Debug)]
pub struct Config {
    #[command(subcommand)]
    pub config_commands: Option<ConfigCommands>,
}

impl Config {
    pub fn handle_config(&self) -> Result<(), anyhow::Error> {
        match &self.config_commands {
            Some(ConfigCommands::Add(args)) => match api_config::add_server(args.clone()) {
                Ok(()) => {
                    println!("Server {} successfully added", args.name);
                    return Ok(());
                }
                Err(error) => return Err(anyhow::anyhow!(error)),
            },
            Some(ConfigCommands::Remove(args)) => match api_config::remove_server(args) {
                Ok(()) => {
                    println!("Server {} successfully removed", args.name);
                    return Ok(());
                }
                Err(error) => return Err(anyhow::anyhow!(error)),
            },
            Some(ConfigCommands::Update(args)) => match api_config::update_server(args) {
                Ok(()) => {
                    println!("Server {} successfully updated", args.name);
                    return Ok(());
                }
                Err(error) => return Err(anyhow::anyhow!(error)),
            },
            Some(ConfigCommands::Use(args)) => match api_config::use_server(args.clone()) {
                Ok(()) => {
                    println!("Using Server {}", args.name);
                    return Ok(());
                }
                Err(error) => return Err(anyhow::anyhow!(error)),
            },
            Some(ConfigCommands::GetServers(_args)) => match api_config::get_servers() {
                Ok(()) => {
                    return Ok(());
                }
                Err(error) => return Err(anyhow::anyhow!(error)),
            },
            None => {
                return Err(anyhow::anyhow!("No valid arguments"));
            }
        }
    }
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
