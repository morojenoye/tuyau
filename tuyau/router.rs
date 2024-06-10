use axum::extract::State;
use ruma::{
	api::{client::error::ErrorKind, federation::query::get_room_information, OutgoingResponse},
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

type GetRoomInfoReply = get_room_information::v1::Response;

pub async fn get_room_information_route<'a, T: QueryExecutor>(
	request: MApi<get_room_information::v1::Request>,
	State(state): State<Executor<'a, T>>,
) -> Result<MApiReply<impl OutgoingResponse>, MApiError<ErrorKind>> {
	// =====================================================================
	if request.body.room_alias != state.setups.room_id {
		return Err(MApiError(ErrorKind::NotFound));
	}
	// =====================================================================
	Ok(MApiReply(GetRoomInfoReply::new(
		owned_room_id!("!ffffffff:stokejo.com"),
		Vec::new(),
	)))
}
