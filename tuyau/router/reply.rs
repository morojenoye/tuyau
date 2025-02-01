use axum::response::{IntoResponse, Response as Reply};
use ruma::api::{client::error::ErrorKind, OutgoingResponse};

pub struct MApiReply<T>(pub T);

impl<T> IntoResponse for MApiReply<T>
where
	T: OutgoingResponse,
{
	fn into_response(self) -> Reply {
		match self.0.try_into_http_response::<bytes::BytesMut>() {
			Ok(r) => r.map(|b| b.freeze().into()),
			Err(error) => panic!("{:?}", error),
		}
	}
}

pub struct MApiError<T>(pub T);

impl IntoResponse for MApiError<ErrorKind> {
	fn into_response(self) -> Reply {
		todo!()
	}
}
