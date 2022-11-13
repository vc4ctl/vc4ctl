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
                    match config::config::add_server(&args.name, &args.url, &args.token ) {
                        Ok(()) => {return Ok(());}
                        Err(error) => return Err(anyhow::anyhow!(error))
                    }                
                }
                Some(ConfigCommands::Remove(args)) => {
                    println!("{}", args.name);
                    
                }
                Some(ConfigCommands::Update(args)) => {
                    println!("{}", args.name);
                }
                Some(ConfigCommands::Use(args)) => {
                    println!("{}", args.name);
                }
                Some(ConfigCommands::GetServers(_args)) => {
                    println!("GetServers")
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
