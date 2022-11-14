mod cli;
mod api;

use clap::{Parser};
use cli::cli::{Cli, Commands, ConfigCommands};
use api::config;


fn main() -> anyhow::Result<()>{
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Config(args)) => {
            match &args.config_commands {
                Some(ConfigCommands::Add(args)) => {
                    match config::config::add_server(args.clone()) {
                        Ok(()) => {
                            println!("Server {} successfully added", args.name);
                            return Ok(());}
                        Err(error) => return Err(anyhow::anyhow!(error))
                    }                
                }
                Some(ConfigCommands::Remove(args)) => {
                    match config::config::remove_server(args) {
                        Ok(()) => {
                            println!("Server {} successfully removed", args.name);
                            return Ok(());
                        }
                        Err(error) => return Err(anyhow::anyhow!(error))
                    }                    
                }
                Some(ConfigCommands::Update(args)) => {
                    match config::config::update_server(args) {
                        Ok(()) => {
                            println!("Server {} successfully updated", args.name);
                            return Ok(());
                        }
                        Err(error) => return Err(anyhow::anyhow!(error))
                    }
                }
                Some(ConfigCommands::Use(args)) => {
                    match config::config::use_server(args.clone()){
                        Ok(()) => {
                            println!("Using Server {}", args.name);
                            return Ok(());
                        }
                        Err(error) => return Err(anyhow::anyhow!(error))
                    }
                }
                Some(ConfigCommands::GetServers(_args)) => {
                    match config::config::get_servers() {
                        Ok(()) => {                            
                            return Ok(());
                        }
                        Err(error) => return Err(anyhow::anyhow!(error))
                    }
                }
                None => {
                    return Err(anyhow::anyhow!("No valid arguments"));
                }
            }
        }
        None => {
            println!("No valid arguments")
        }
    }

    Ok(())
}
