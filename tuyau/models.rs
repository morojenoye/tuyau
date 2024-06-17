use ruma::{events::pdu::RoomV3Pdu, RoomAliasId, RoomId, UserId};
use sea_orm::{
	ActiveModelBehavior, ActiveModelTrait, ConnectionTrait, DatabaseConnection, DeriveEntityModel,
	DerivePrimaryKey, DeriveRelation, EntityTrait, EnumIter, IntoActiveModel, PrimaryKeyTrait,
	Schema,
};

use crate::{
	models,
	worker::{QueryExecutor, SetupBundle},
	Maybe, MyResult,
};

pub mod keyserver;
pub mod state;
pub mod timeline;

pub type MaybePdu = Maybe<RoomV3Pdu>;

// =========================================================================

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
			query.create_table_from_entity(models::Entity),
		] {
			let statement = backend.build(statement.if_not_exists());
			inner.execute(statement).await?;
		}
		Ok(DefaultQueryExecutor { inner })
	}
}

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

impl QueryExecutor for DefaultQueryExecutor {
	async fn new(&self, setup: SetupBundle) -> MyResult<()> {
		// =================================================================
		let setup: ActiveModel = Model {
			alias: setup.alias.to_string(),
			admin: setup.admin.to_string(),
			ident: setup.ident.to_string(),
		}
		.into_active_model();

		setup.insert(&self.inner).await?;
		Ok(())
	}

	async fn get(&self) -> MyResult<Maybe<SetupBundle>> {
		// =================================================================
		let setup: _ = Entity::find().one(&self.inner);

		let Some(setup) = setup.await? else {
			return Ok(None);
		};
		let setup = SetupBundle {
			alias: RoomAliasId::parse(setup.alias)?,
			admin: UserId::parse(setup.admin)?,
			ident: RoomId::parse(setup.ident)?,
		};
		Ok(Some(setup))
	}
}
