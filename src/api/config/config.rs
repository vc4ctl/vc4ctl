use std::{path::{PathBuf, Path}, any};
use serde::{Serialize, Deserialize};
use anyhow::{Context, Result, anyhow};

#[derive(Serialize, Deserialize, Debug)]
pub struct Server {
  url: String,
  name: String,
  token: String
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Config {
  #[serde(default)]
  #[serde(rename = "current-server")]
  current_server: String,

  #[serde(default)]
  servers: Vec<Server>
}

pub fn add_server(name: &str, url:&str, token:&str) -> anyhow::Result<()>{
  let mut config = get_config()?;

  let new_server = Server {
    url: url.to_string(),
    name: name.to_string(),
    token: token.to_string()
  };

  config.servers.push(new_server);    

  return save_config(&config);
}

pub fn update_server(name: &str, host: &str, token: &str){
  println!("{} {} {}", name, host, token)
}

fn save_config(config: &Config) -> Result<()> {
  let contents = serde_yaml::to_string(config)?;

  let result = match home::home_dir() {
    Some(home) => {
      let path = home.join(".vc4/config.yaml").clone();

      match std::fs::write(path, contents) {
        Ok(()) => Ok(()),
        Err(error) => Err(anyhow!(error))
      }
    },
    None => Err(anyhow!("Unable to find home directory"))
  };
  result
}

fn get_config() -> Result<Config> {
  let config = match get_config_file_exists() {
    Some(config_path) => {
      let yaml = std::fs::read_to_string(&config_path)
      .with_context(|| "Found config.yaml, but unable to read the contents")?;

      let mut config = match yaml.trim().is_empty() {
        true => Config {
          ..Default::default()
        },
        false => serde_yaml::from_str(yaml.as_str()).with_context(|| "Found config.yaml, but unable to deserialize")?,
      };

      config
    },
    None => Config {
      ..Default::default()
    }
  };

  Ok(config)
}

fn get_config_file_exists() -> Option<PathBuf> {
  if let Some(home) = home::home_dir() {
    let config = home.join(".vc4/config.yaml");

    if config.is_file() {
      return Some(config);
    }
  }

  None
}