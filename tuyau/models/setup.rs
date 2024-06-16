use sea_orm::{
	ActiveModelBehavior, DeriveEntityModel, DerivePrimaryKey, DeriveRelation, EntityTrait,
	EnumIter, PrimaryKeyTrait,
};

use crate::{models::DefaultQueryExecutor, worker::setup};

// =========================================================================

#[derive(Clone, Debug, DeriveEntityModel)]
#[sea_orm(table_name = "setup")]
pub struct Model {
	#[sea_orm(primary_key, unique, auto_increment = false)]
	alias: String,
	admin: String,
	ident: String,
}

#[derive(Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

// =========================================================================

impl setup::QueryExecutor for DefaultQueryExecutor {}
