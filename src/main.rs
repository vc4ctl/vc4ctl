mod api;
mod cli;

use api::config::{add_server, remove_server, update_server, use_server, get_servers};
use clap::Parser;
use cli::{Cli, Commands};
use cli::config::{ConfigCommands};
use tracing::metadata::LevelFilter;
use tracing::{Level, Subscriber};
use tracing_subscriber::FmtSubscriber;
use log;

fn configure_tracing(args:&Cli) -> impl Subscriber {
    let builder = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .with_target(false)
        .without_time();

    match args.verbose.log_level_filter() {
        log::LevelFilter::Debug => builder.with_max_level(LevelFilter::DEBUG),
        log::LevelFilter::Error => builder.with_max_level(LevelFilter::ERROR),
        log::LevelFilter::Info => builder.with_max_level(LevelFilter::INFO),
        log::LevelFilter::Off => builder.with_max_level(LevelFilter::OFF),
        log::LevelFilter::Trace => builder.with_max_level(LevelFilter::TRACE),
        log::LevelFilter::Warn => builder.with_max_level(LevelFilter::WARN)
    }.finish()
}

fn main() -> anyhow::Result<()> {  
    let cli = Cli::parse();

    let subscriber = configure_tracing(&cli);

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    match &cli.command {
        Some(Commands::Config(args)) => match &args.config_commands {
            Some(ConfigCommands::Add(args)) => match add_server(args.clone()) {
                Ok(()) => {
                    println!("Server {} successfully added", args.name);
                    return Ok(());
                }
                Err(error) => return Err(anyhow::anyhow!(error)),
            },
            Some(ConfigCommands::Remove(args)) => match remove_server(args) {
                Ok(()) => {
                    println!("Server {} successfully removed", args.name);
                    return Ok(());
                }
                Err(error) => return Err(anyhow::anyhow!(error)),
            },
            Some(ConfigCommands::Update(args)) => match update_server(args) {
                Ok(()) => {
                    println!("Server {} successfully updated", args.name);
                    return Ok(());
                }
                Err(error) => return Err(anyhow::anyhow!(error)),
            },
            Some(ConfigCommands::Use(args)) => match use_server(args.clone()) {
                Ok(()) => {
                    println!("Using Server {}", args.name);
                    return Ok(());
                }
                Err(error) => return Err(anyhow::anyhow!(error)),
            },
            Some(ConfigCommands::GetServers(_args)) => match get_servers() {
                Ok(()) => {
                    return Ok(());
                }
                Err(error) => return Err(anyhow::anyhow!(error)),
            },
            None => {
                return Err(anyhow::anyhow!("No valid arguments"));
            }
        },
        None => {
            println!("No valid arguments")
        }
    }

    Ok(())
}
