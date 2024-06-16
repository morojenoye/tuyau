use ruma::{events::pdu::RoomV3Pdu, EventId};

use crate::{models::MaybePdu, MyResult, Ref};

pub trait QueryExecutor {
	async fn new(&self, pdu_id: &EventId, pdu: &RoomV3Pdu) -> MyResult<()>;
	async fn get(&self, pdu_id: &EventId) -> MyResult<MaybePdu>;
}

#[derive(Clone)]
pub struct Executor<T: QueryExecutor> {
	pub(super) state: Ref<T>,
}

impl<T: QueryExecutor> Executor<T> {
	pub async fn new(&self, pdu_id: &EventId, pdu: &RoomV3Pdu) {}
	pub async fn get(&self, pdu_id: &EventId) {}
}
