use axum::extract::State;
use ruma::{
	api::{
		client::error::ErrorKind,
		federation::query::get_room_information::v1::{
			Request as GetRoomInfoRequest, Response as GetRoomInfoReply,
		},
		OutgoingResponse,
	},
	owned_room_id,
};

use crate::{
	router::{
		extract::MApi,
		reply::{MApiError, MApiReply},
	},
	worker::{Executor, QueryExecutor},
};

pub mod extract;
pub mod reply;

pub async fn get_room_information_route<'a, T: QueryExecutor>(
	State(ctx): State<Executor<'a, T>>,
	req: MApi<GetRoomInfoRequest>,
) -> Result<MApiReply<impl OutgoingResponse>, MApiError<ErrorKind>> {
	// =====================================================================
	if req.body.room_alias != ctx.setups.room_id {
		return Err(MApiError(ErrorKind::NotFound));
	}
	// =====================================================================
	Ok(MApiReply(GetRoomInfoReply::new(
		owned_room_id!("!ffffffff:stokejo.com"),
		Vec::new(),
	)))
}
