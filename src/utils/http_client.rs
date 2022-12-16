use reqwest::header::{ACCEPT, AUTHORIZATION};
use reqwest::blocking::Client;
use serde::de::DeserializeOwned;
use anyhow::{Context, Result};

use crate::api::config::Server;



pub fn get<T:DeserializeOwned>(server: &Server, path: &str) -> Result<T>
{
    let url = format!("{}/{}", server.url, path);

    let client = Client::new();

    let res = client
        .get(url)
        .header(AUTHORIZATION, server.token.as_str())
        .header(ACCEPT, "application/json")
        .send()
        .with_context(|| "Error occured while sending request")?;

    let response: T = match res.json() {
        Ok(response) => response,
        Err(error) => return Err(anyhow::anyhow!("Unable to deserialize response; {}", error)),
    };

    Ok(response)
}