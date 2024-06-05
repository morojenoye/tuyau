use ruma::ServerName;
use sea_orm::{
	ActiveModelBehavior, DeriveEntityModel, DerivePrimaryKey, DeriveRelation, EntityTrait,
	EnumIter, PrimaryKeyTrait,
};

use crate::{
	models::{QueryExecutor, ServerKeys},
	worker::keyserver,
	MyResult,
};

// =========================================================================

#[derive(Clone, Debug, DeriveEntityModel)]
#[sea_orm(table_name = "keyserver")]
pub struct Model {
	#[sea_orm(primary_key, unique)]
	pub server: String,
	pub keys: String,
}

#[derive(Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

// =========================================================================

impl keyserver::Query for QueryExecutor {
	fn get_server_keys(server: &ServerName) -> MyResult<ServerKeys> {
		todo!()
	}
}
