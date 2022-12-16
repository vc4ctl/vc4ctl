use crate::cli::room_control::RoomControlArgs;
use crate::utils::constants::DEFAULT_PATH;
use crate::utils::http_client::put;
use anyhow::{Context, Result};
use reqwest::blocking::multipart::Form;

use super::config;

pub fn stop_room(args: &RoomControlArgs) -> Result<()> {
    let server =
        config::get_current_server().with_context(|| "unable to get server configuration")?;

    let form = Form::new()
        .text("ProgramInstanceId", args.room_id.to_string())
        .text("Stop", "true");

    let url = format!("{}{}", DEFAULT_PATH, "ProgramInstance");

    print!("Stopping room {}...", args.room_id);

    put(&server, url.as_str(), form).with_context(|| "Error occured while starting program")?;

    println!("...stopped!");
    Ok(())
}

pub fn start_room(args: &RoomControlArgs) -> Result<()> {
    let server =
        config::get_current_server().with_context(|| "unable to get server configuration")?;

    let form = Form::new()
        .text("ProgramInstanceId", args.room_id.to_string())
        .text("Start", "true");

    let url = format!("{}{}", DEFAULT_PATH, "ProgramInstance");

    print!("Starting room {}...", args.room_id);

    put(&server, url.as_str(), form).with_context(|| "Error occured while starting program")?;

    println!("...started!");

    Ok(())
}
