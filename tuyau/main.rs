use ruma::events::pdu::RoomV3Pdu;

mod confconf;
mod executor;
mod models;
mod router;

pub type MyResult<T> = anyhow::Result<T>;
pub type MaybePdu = Option<RoomV3Pdu>;

#[tokio::main]
async fn main() {}
