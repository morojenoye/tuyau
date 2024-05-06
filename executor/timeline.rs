use ruma::{events::pdu::RoomV3Pdu, EventId};

use crate::{MaybePdu, MyResult};

pub trait Query {
	async fn append(&self, pdu_id: &EventId, pdu: &RoomV3Pdu) -> MyResult<()>;
	async fn select(&self, pdu_id: &EventId) -> MyResult<MaybePdu>;
}

pub struct Executor<'a, T: Query> {
	inner: &'a T,
}

impl<'a, T: Query> Executor<'a, T> {
	pub async fn append(&self, pdu_id: &EventId, pdu: &RoomV3Pdu) {}
	pub async fn select(&self, pdu_id: &EventId) {}
}
