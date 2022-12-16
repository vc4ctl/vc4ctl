use std::collections::HashMap;

use serde::Deserialize;
use tabled::Tabled;

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

    #[serde(default)]
    #[serde(rename = "ProgramLibrary")]
    pub program_library: Option<HashMap<u32, Program>>,
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

#[derive(Deserialize, Debug, Tabled, Default, Clone)]
pub struct Program {
    #[serde(rename = "ProgramId")]
    program_id: u64,
    #[serde(rename = "FriendlyName")]
    friendly_name: String,
    #[serde(rename = "Notes")]
    notes: String,
    #[serde(rename = "AppFile")]
    app_file: String,
    #[serde(rename = "AppFileTS")]
    app_file_ts: String,
    #[serde(rename = "MobilityFile")]
    mobility_file: String,
    #[serde(rename = "MobilityFileTS")]
    mobility_file_ts: String,
    #[serde(rename = "WebxPanelFile")]
    web_xpanel_file: String,
    #[serde(rename = "WebxPanelFileTS")]
    web_xpanel_file_ts: String,
    #[serde(rename = "ProjectFile")]
    project_file: String,
    #[serde(rename = "ProjectFileTS")]
    project_file_ts: String,
    #[serde(rename = "CwsFile")]
    cws_file: String,
    #[serde(rename = "CwsFileTS")]
    cws_file_ts: String,
    #[serde(rename = "ProgramType")]
    program_type: String,
    #[serde(rename = "ProgramName")]
    program_name: String,
    #[serde(rename = "CompileDateTime")]
    compile_date_time: String,
    #[serde(rename = "CresDBVersion")]
    cres_db_version: String,
    #[serde(rename = "DeviceDBVersion")]
    device_db_version: String,
    #[serde(rename = "IncludeDatVersion")]
    include_dat_version: String,
}
