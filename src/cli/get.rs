use clap::{Args, ValueEnum};

#[derive(Args, Debug)]
pub struct GetArgs {
    #[arg(value_enum, help = "resource type")]
    pub resource: ResourceTypes,

    #[arg(help = "Resource ")]
    pub resource_id: Option<String>,

    #[arg(short = 'w', long = "wide", help = "Show all fields")]
    pub wide: bool,
}

#[derive(Debug, Clone, ValueEnum)]
pub enum ResourceTypes {
    AuthenticationGroups,
    AuthenticationGroup,
    Devices, // DeviceInfo in REST API
    Device,
    DeviceMaps,
    DeviceMap,
    DeviceProgramMaps,
    DeviceProgramMap,
    Ethernet,
    Rooms,
    Room,
    Programs,
    Program,
}
