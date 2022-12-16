use std::collections::HashMap;

use crate::api::config;
use crate::cli::get::{GetArgs, ResourceTypes};
use crate::utils::constants::DEFAULT_PATH;
use crate::utils::http_client::get;
use anyhow::{Context, Result};
use reqwest::header::{ACCEPT, AUTHORIZATION};
use tabled::locator::ByColumnName;
use tabled::{Disable, Style};

use crate::utils::vc4_entities::{Program, ProgramInstance, Response};

use super::config::Server;

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
        ResourceTypes::Programs => get_programs(&server, &args.resource_id, &args.wide),
        ResourceTypes::Program => get_programs(&server, &args.resource_id, &args.wide),
    }
}

fn get_rooms(server: &Server, room_id: &Option<String>, wide: &bool) -> Result<()> {
    let path = match room_id {
        Some(id) => format!("{}{}/{}", DEFAULT_PATH, "ProgramInstance", id),
        None => format!("{}{}", DEFAULT_PATH, "ProgramInstance"),
    };

    let response: Response = match get(server, &path) {
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

fn get_programs(server: &Server, room_id: &Option<String>, wide: &bool) -> Result<()> {
    let url = match room_id {
        Some(id) => format!("{}{}{}/{}", server.url, DEFAULT_PATH, "ProgramLibrary", id),
        None => format!("{}{}{}", server.url, DEFAULT_PATH, "ProgramLibrary"),
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

    let programs: HashMap<u32, Program> = match response.device.programs.program_library {
        Some(instances) => instances,
        None => HashMap::new(),
    };

    let program_vec: Vec<Program> = programs.values().cloned().collect();

    if !wide {
        let table = tabled::Table::new(program_vec)
            .with(Style::blank())
            .with(Disable::column(ByColumnName::new("program_id")))
            .with(Disable::column(ByColumnName::new("notes")))
            .with(Disable::column(ByColumnName::new("mobility_file")))
            .with(Disable::column(ByColumnName::new("mobility_file_ts")))
            .with(Disable::column(ByColumnName::new("web_xpanel_file")))
            .with(Disable::column(ByColumnName::new("web_xpanel_file_ts")))
            .with(Disable::column(ByColumnName::new("project_file")))
            .with(Disable::column(ByColumnName::new("project_file_ts")))
            .with(Disable::column(ByColumnName::new("cws_file")))
            .with(Disable::column(ByColumnName::new("cws_file_ts")))
            .with(Disable::column(ByColumnName::new("compile_date_time")))
            .with(Disable::column(ByColumnName::new("cres_db_version")))
            .with(Disable::column(ByColumnName::new("device_db_version")))
            .with(Disable::column(ByColumnName::new("include_dat_version")))
            .with(Disable::column(ByColumnName::new("app_file_ts")))
            .to_string();

        print!("{}", table);
    } else {
        let table = tabled::Table::new(program_vec)
            .with(Style::blank())
            .to_string();

        print!("{}", table);
    }

    Ok(())
}
