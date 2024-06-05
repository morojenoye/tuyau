use std::collections::BTreeMap;

use ruma::{
	api::federation::discovery::VerifyKey, events::pdu::RoomV3Pdu, OwnedServerSigningKeyId,
};

pub mod keyserver;
pub mod state;
pub mod timeline;

pub type ServerKeys = BTreeMap<OwnedServerSigningKeyId, VerifyKey>;
pub type MaybePdu = Option<RoomV3Pdu>;

pub struct QueryExecutor {
	inner: sea_orm::DatabaseConnection,
}
