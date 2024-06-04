use ruma::events::pdu::RoomV3Pdu;

mod models;
mod router;
mod setups;
mod worker;

pub type MyResult<T> = anyhow::Result<T>;
pub type MaybePdu = Option<RoomV3Pdu>;

pub struct Global {
	server_name: String,
}

#[tokio::main]
async fn main() {}
