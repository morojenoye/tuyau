use async_trait::async_trait;

use ruma::{api::federation::discovery::ServerSigningKeys, ServerName};
use sea_orm::{
	ActiveModelBehavior, DeriveEntityModel, DerivePrimaryKey, DeriveRelation, EntityTrait,
	EnumIter, PrimaryKeyTrait,
};

use crate::{models::DefaultQueryExecutor, worker::keyserver, MyResult};

// =========================================================================

#[derive(Clone, Debug, DeriveEntityModel)]
#[sea_orm(table_name = "keyserver")]
pub struct Model {
	#[sea_orm(primary_key, unique, auto_increment = false)]
	pub server: String,
	pub keys: String,
}

#[derive(Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

// =========================================================================

#[async_trait]
impl keyserver::QueryExecutor for DefaultQueryExecutor {
	async fn get(&self, server: &ServerName) -> MyResult<ServerSigningKeys> {
		todo!()
	}
}
