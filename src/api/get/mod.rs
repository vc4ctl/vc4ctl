use std::collections::HashMap;

use crate::api::config;
use crate::cli::get::{GetArgs, ResourceTypes};
use anyhow::{Context, Result};
use reqwest::header::{ACCEPT, AUTHORIZATION};
use serde::Deserialize;
use tabled::locator::ByColumnName;
use tabled::{Disable, Style, Tabled};

use super::config::Server;

const DEFAULT_PATH: &str = "/VirtualControl/config/api/";

#[derive(Deserialize, Debug, Default)]
pub struct Response {
    #[serde(default)]
    #[serde(rename = "Device")]
    pub device: Device,
}

#[derive(Deserialize, Debug, Default)]
pub struct Device {
    #[serde(default)]
    #[serde(rename = "Programs")]
    pub programs: Programs,
}

#[derive(Deserialize, Debug, Default)]
pub struct Programs {
    #[serde(default)]
    #[serde(rename = "ProgramInstanceLibrary")]
    pub program_instance_library: Option<HashMap<String, ProgramInstance>>,
}

#[derive(Deserialize, Debug, Tabled, Default, Clone)]
pub struct ProgramInstance {
    id: u64,
    #[serde(rename = "ProgramInstanceId")]
    program_instance_id: String,
    #[serde(rename = "UserFile")]
    user_file: String,
    #[serde(rename = "Status")]
    status: String,
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "ProgramLibraryId")]
    program_library_id: u64,
    #[serde(rename = "Level")]
    level: String,
    #[serde(rename = "AddressSetsLocation")]
    address_sets_location: bool,
    #[serde(rename = "Location")]
    location: String,
    #[serde(rename = "Longitude")]
    longitude: String,
    #[serde(rename = "Latitude")]
    latitude: String,
    #[serde(rename = "TimeZone")]
    time_zone: String,
    #[serde(rename = "ConfigurationLink")]
    configuration_link: String,
    #[serde(rename = "XpanelUrl")]
    xpanel_url: String,
    #[serde(rename = "Notes")]
    notes: String,
    #[serde(rename = "DebuggingEnabled")]
    debugging_enabled: bool,
}

pub fn get_resource(args: &GetArgs) -> Result<()> {
    let server =
        config::get_current_server().with_context(|| "unable to get server configuration")?;

    match &args.resource {
        ResourceTypes::AuthenticationGroups => todo!(),
        ResourceTypes::AuthenticationGroup => todo!(),
        ResourceTypes::Devices => todo!(),
        ResourceTypes::Device => todo!(),
        ResourceTypes::DeviceMaps => todo!(),
        ResourceTypes::DeviceMap => todo!(),
        ResourceTypes::DeviceProgramMaps => todo!(),
        ResourceTypes::DeviceProgramMap => todo!(),
        ResourceTypes::Ethernet => todo!(),
        ResourceTypes::Rooms => get_rooms(&server, &args.resource_id, &args.wide),
        ResourceTypes::Room => get_rooms(&server, &args.resource_id, &args.wide),
        ResourceTypes::Programs => todo!(),
        ResourceTypes::Program => todo!(),
    }
}

fn get_rooms(server: &Server, room_id: &Option<String>, wide: &bool) -> Result<()> {
    let url = match room_id {
        Some(id) => format!("{}{}{}/{}", server.url, DEFAULT_PATH, "ProgramInstance", id),
        None => format!("{}{}{}", server.url, DEFAULT_PATH, "ProgramInstance"),
    };

    let client = reqwest::blocking::Client::new();

    let res = client
        .get(url)
        .header(AUTHORIZATION, server.token.as_str())
        .header(ACCEPT, "application/json")
        .send()
        .with_context(|| "Error occured while sending request")?;

    let response: Response = match res.json() {
        Ok(response) => response,
        Err(error) => return Err(anyhow::anyhow!("Unable to deserialize response; {}", error)),
    };

    let rooms: HashMap<String, ProgramInstance> =
        match response.device.programs.program_instance_library {
            Some(instances) => instances,
            None => HashMap::new(),
        };

    let room_vec: Vec<ProgramInstance> = rooms.values().cloned().collect();

    if !wide {
        let table = tabled::Table::new(room_vec)
            .with(Style::blank())
            .with(Disable::column(ByColumnName::new("id")))
            .with(Disable::column(ByColumnName::new("user_file")))
            .with(Disable::column(ByColumnName::new("program_library_id")))
            .with(Disable::column(ByColumnName::new("level")))
            .with(Disable::column(ByColumnName::new("address_sets_location")))
            .with(Disable::column(ByColumnName::new("location")))
            .with(Disable::column(ByColumnName::new("longitude")))
            .with(Disable::column(ByColumnName::new("latitude")))
            .with(Disable::column(ByColumnName::new("time_zone")))
            .with(Disable::column(ByColumnName::new("notes")))
            .to_string();

        print!("{}", table);
    } else {
        let table = tabled::Table::new(room_vec)
            .with(Style::blank())
            .to_string();

        print!("{}", table);
    }

    Ok(())
}
