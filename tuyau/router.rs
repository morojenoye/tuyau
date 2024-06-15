use axum::extract::State;
use ruma::{
	api::{
		client::error::ErrorKind,
		federation::query::get_room_information::v1::{
			Request as GetRoomInfoRequest, Response as GetRoomInfoReply,
		},
		OutgoingResponse,
	},
	owned_room_id, owned_server_name,
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

pub async fn get_room_information_route<T: QueryExecutor>(
	State(ctx): State<Executor<T>>,
	req: MApi<GetRoomInfoRequest>,
) -> Result<MApiReply<impl OutgoingResponse>, MApiError<ErrorKind>> {
	// =====================================================================
	if req.body.room_alias != ctx.alias {
		return Err(MApiError(ErrorKind::NotFound));
	}
	// =====================================================================
	Ok(MApiReply(GetRoomInfoReply::new(
		ctx.ident.clone(),
		vec![ctx.server_name()],
	)))
}
