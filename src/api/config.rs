use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tabled::{Style, Tabled};
use tracing::instrument;

use crate::cli::config::{ConfigAddArgs, ConfigRemoveArgs, ConfigUpdateArgs, ConfigUseArgs};

#[derive(Serialize, Deserialize, Debug, Tabled, Clone)]
pub struct Server {
    pub url: String,
    name: String,
    pub token: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Config {
    #[serde(default)]
    #[serde(rename = "current-server")]
    current_server: String,

    #[serde(default)]
    servers: Vec<Server>,
}

#[instrument(skip(args))]
pub fn add_server(args: ConfigAddArgs) -> Result<()> {
    let mut config = get_config().with_context(|| "Unable to retrieve config")?;

    if config.servers.iter().any(|n| n.name == args.name) {
        return Err(anyhow!(
            "Unable to add server with name {}. Server with that name already exists",
            args.name
        ));
    }

    let new_server = Server {
        url: args.url,
        name: args.name,
        token: args.token,
    };

    config.servers.push(new_server);

    save_config(&config)
}

#[instrument(skip(args))]
pub fn remove_server(args: &ConfigRemoveArgs) -> Result<()> {
    let mut config = get_config().with_context(|| "Unable to retrieve config")?;

    if config.current_server == args.name {
        config.current_server = "".to_string();
    }

    config.servers.retain(|s| s.name != args.name);

    save_config(&config)
}

#[instrument(skip(args))]
pub fn update_server(args: &ConfigUpdateArgs) -> Result<()> {
    let mut config = get_config().with_context(|| "Unable to retrieve config")?;

    let mut current_server = config.servers.remove(
        config
            .servers
            .iter()
            .position(|s| s.name == args.name)
            .with_context(|| format!("Server with name {} not found", args.name))?,
    );

    match &args.url {
        Some(url) => current_server.url = url.to_string(),
        None => (),
    }

    match &args.token {
        Some(token) => current_server.token = token.to_string(),
        None => (),
    }

    config.servers.push(current_server);

    save_config(&config)
}

#[instrument(skip(args))]
pub fn use_server(args: ConfigUseArgs) -> Result<()> {
    let mut config = get_config().with_context(|| "Unable to retrieve config")?;

    if !config.servers.iter().any(|s| s.name == args.name) {
        return Err(anyhow!(
            "Server with name {} does not exist. Please add it before trying to use it.",
            args.name
        ));
    }

    config.current_server = args.name;

    save_config(&config)
}

#[instrument()]
pub fn get_servers() -> Result<()> {
    let config = get_config().with_context(|| "Unable to retrieve config")?;

    let table = tabled::Table::new(config.servers)
        .with(Style::modern())
        .to_string();

    print!("{}", table);

    Ok(())
}

#[instrument()]
pub fn get_current_server() -> Result<Server> {
    let config = get_config().with_context(|| "Unable to rerieve config")?;

    if config.current_server.is_empty() {
        return Err(anyhow!(
            "No server to use. Use `vc4ctl config use <NAME>` to set a server to use"
        ));
    }

    let server_to_use = match config
        .servers
        .iter()
        .find(|s| s.name == config.current_server)
        .cloned()
    {
        Some(s) => s,
        None => {
            return Err(anyhow!(
                "Current server {} is not in configuration!",
                config.current_server
            ))
        }
    };

    Ok(server_to_use)
}

#[instrument(skip(config))]
fn save_config(config: &Config) -> Result<()> {
    let contents = serde_yaml::to_string(config)?;

    let result = match home::home_dir() {
        Some(home) => {
            let config_folder = home.join(".vc4");

            let path = config_folder.join("config.yaml");

            if !config_folder.exists() {
                std::fs::create_dir(config_folder.clone())
                    .with_context(|| "Unable to create config directory")?;
            }

            if !path.exists() {
                std::fs::File::create(path.clone())
                    .with_context(|| "Unable to create config file")?;
            }

            match std::fs::write(path, contents) {
                Ok(()) => Ok(()),
                Err(error) => Err(anyhow!("Unable to write config: {}", error)),
            }
        }
        None => Err(anyhow!("Unable to find home directory")),
    };

    result
}

#[instrument()]
fn get_config() -> Result<Config> {
    let config = match get_config_file_exists() {
        Some(config_path) => {
            let yaml = std::fs::read_to_string(&config_path)
                .with_context(|| "Found config.yaml, but unable to read the contents")?;

            let config = match yaml.trim().is_empty() {
                true => Config {
                    ..Default::default()
                },
                false => serde_yaml::from_str(yaml.as_str())
                    .with_context(|| "Found config.yaml, but unable to deserialize")?,
            };

            config
        }
        None => Config {
            ..Default::default()
        },
    };

    Ok(config)
}

#[instrument()]
fn get_config_file_exists() -> Option<PathBuf> {
    if let Some(home) = home::home_dir() {
        let config = home.join(".vc4/config.yaml");

        if config.is_file() {
            return Some(config);
        }
    }

    None
}
