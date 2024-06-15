use ruma::events::pdu::RoomV3Pdu;
use sea_orm::{ConnectionTrait, DatabaseConnection, Schema};

use crate::{models, worker::QueryExecutor, MyResult};

pub mod keyserver;
pub mod setup;
pub mod state;
pub mod timeline;

pub type MaybePdu = Option<RoomV3Pdu>;

#[derive(Clone)]
pub struct DefaultQueryExecutor {
	inner: DatabaseConnection,
}

impl DefaultQueryExecutor {
	pub async fn new(inner: DatabaseConnection) -> MyResult<Self> {
		let backend = inner.get_database_backend();
		let query = Schema::new(backend);

		for mut statement in [
			query.create_table_from_entity(models::keyserver::Entity),
			query.create_table_from_entity(models::timeline::Entity),
		] {
			let statement = backend.build(statement.if_not_exists());
			inner.execute(statement).await?;
		}
		Ok(DefaultQueryExecutor { inner })
	}
}

impl QueryExecutor for DefaultQueryExecutor {}
