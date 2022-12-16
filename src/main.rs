mod api;
mod cli;
mod utils;


use api::get::get_resource;
use clap::Parser;

use cli::{Cli, Commands};
use log;
use std::result::Result::Ok;
use tracing::metadata::LevelFilter;
use tracing::{Level, Subscriber};
use tracing_subscriber::FmtSubscriber;

fn configure_tracing(args: &Cli) -> impl Subscriber {
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
        log::LevelFilter::Warn => builder.with_max_level(LevelFilter::WARN),
    }
    .finish()
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let subscriber = configure_tracing(&cli);

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    let result = match &cli.command {
        Some(Commands::Config(args)) => args.handle_config(),
        Some(Commands::Get(args)) => match get_resource(args) {
            Ok(()) => return Ok(()),
            Err(error) => return Err(anyhow::anyhow!(error)),
        },
        None => {
            return Err(anyhow::anyhow!("No command provided"));
        }
    };

    result
}
