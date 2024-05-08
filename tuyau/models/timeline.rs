use ruma::{events::pdu::RoomV3Pdu, EventId};
use sea_orm::{
	ActiveModelBehavior, ActiveModelTrait, ActiveValue, DeriveEntityModel, DerivePrimaryKey,
	DeriveRelation, EntityTrait, EnumIter, PrimaryKeyTrait,
};

use crate::{executor::timeline, models::QueryExecutor, MaybePdu, MyResult};

// =========================================================================

#[derive(Clone, Debug, DeriveEntityModel)]
#[sea_orm(table_name = "pdu")]
pub struct Model {
	#[sea_orm(primary_key, unique)]
	pub pdu_id: String,
	pub pdu: String,
}

#[derive(Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

// =========================================================================

impl timeline::Query for QueryExecutor {
	async fn append(&self, pdu_id: &EventId, pdu: &RoomV3Pdu) -> MyResult<()> {
		let pdu = serde_json::to_string(pdu)?;

		let pdu = ActiveModel {
			pdu_id: ActiveValue::Set(pdu_id.to_string()),
			pdu: ActiveValue::Set(pdu),
		};
		pdu.insert(&self.inner).await?;

		Ok(())
	}
	async fn select(&self, pdu_id: &EventId) -> MyResult<MaybePdu> {
		let statement = Entity::find_by_id(pdu_id.to_string());

		let pdu = statement.one(&self.inner).await?;

		Ok(pdu.map(|pdu| serde_json::from_str(&pdu.pdu).unwrap()))
	}
}
