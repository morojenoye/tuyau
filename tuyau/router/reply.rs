use axum::response::{IntoResponse, Response as Reply};
use ruma::api::client::error::ErrorKind;

pub struct MApiReply<T>(pub T);

impl IntoResponse for MApiReply<ErrorKind> {
	fn into_response(self) -> Reply {
		todo!()
	}
}
