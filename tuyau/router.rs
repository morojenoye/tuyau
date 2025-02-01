use axum::extract::State;
use ruma::api::{
	client::error::ErrorKind,
	federation::query::get_room_information::v1::{
		Request as GetRoomInfoRequest, Response as GetRoomInfoReply,
	},
	OutgoingResponse,
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
	State(context): State<Executor<T>>,
	req: MApi<GetRoomInfoRequest>,
) -> Result<MApiReply<impl OutgoingResponse>, MApiError<ErrorKind>> {
	// =====================================================================
	if req.body.room_alias != context.alias {
		return Err(MApiError(ErrorKind::NotFound));
	}
	// =====================================================================
	Ok(MApiReply(GetRoomInfoReply::new(
		context.ident.clone(),
		vec![context.server_name()],
	)))
}
