use ruma::{events::pdu::RoomV3Pdu, EventId};

use crate::{models::MaybePdu, MyResult};

pub trait QueryExecutor {
	async fn append(&self, pdu_id: &EventId, pdu: &RoomV3Pdu) -> MyResult<()>;
	async fn select(&self, pdu_id: &EventId) -> MyResult<MaybePdu>;
}

pub struct Executor<'a, T: QueryExecutor> {
	pub(super) query_executor: &'a T,
}

impl<'a, T: QueryExecutor> Executor<'a, T> {
	pub async fn append(&self, pdu_id: &EventId, pdu: &RoomV3Pdu) {}
	pub async fn select(&self, pdu_id: &EventId) {}
}
