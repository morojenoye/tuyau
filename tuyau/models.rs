use {ruma::events::pdu, sea_orm::DatabaseConnection};

use crate::worker::QueryExecutor;

pub mod keyserver;
pub mod state;
pub mod timeline;

pub type MaybePdu = Option<pdu::RoomV3Pdu>;

#[derive(Clone)]
pub struct DefaultQueryExecutor {
	inner: DatabaseConnection,
}

impl DefaultQueryExecutor {
	pub fn new(inner: DatabaseConnection) -> Self {
		DefaultQueryExecutor { inner }
	}
}

impl QueryExecutor for DefaultQueryExecutor {}
