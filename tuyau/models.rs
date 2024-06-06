use ruma::events::pdu::RoomV3Pdu;

use crate::worker::QueryExecutor;

pub mod keyserver;
pub mod state;
pub mod timeline;

pub type MaybePdu = Option<RoomV3Pdu>;

pub struct DefaultQueryExecutor {
	inner: sea_orm::DatabaseConnection,
}

impl QueryExecutor for DefaultQueryExecutor {}
