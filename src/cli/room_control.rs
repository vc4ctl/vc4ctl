use clap::{Args};

#[derive(Args, Debug)]
pub struct RoomControlArgs {
    #[arg(help = "Room ID")]
    pub room_id: String,
}