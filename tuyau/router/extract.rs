use std::collections::BTreeMap;

use async_trait::async_trait;
use axum::{
	body::Body,
	extract::{FromRequest, FromRequestParts, Path},
	http::Request,
	response::{IntoResponse, Response},
	RequestExt, RequestPartsExt,
};
use axum_extra::{headers::Authorization, TypedHeader};
use ruma::{
	api::{
		client::error::{ErrorBody, ErrorKind},
		AuthScheme, IncomingRequest, OutgoingResponse,
	},
	server_util::authorization::XMatrix,
	CanonicalJsonValue,
};
use {bytes::BytesMut, http::StatusCode};

type AuthHeader = Authorization<XMatrix>;
type PathArgs = Vec<String>;

pub struct Ruma<T> {
	pub body: T,
}

#[async_trait]
impl<T, S> FromRequest<S> for Ruma<T>
where
	T: IncomingRequest,
{
	type Rejection = Response;

	async fn from_request(req: Request<Body>, _: &S) -> Result<Self, Self::Rejection> {
		let AuthScheme::ServerSignatures = T::METADATA.authentication else {
			let error_body = ErrorBody::Standard {
				kind: ErrorKind::Unauthorized,
				message: String::new(),
			};
			let error = error_body.into_error(StatusCode::UNAUTHORIZED);

			let Ok(response) = error.try_into_http_response::<BytesMut>() else {
				return Err(StatusCode::INTERNAL_SERVER_ERROR.into_response());
			};
			return Err(response.map(BytesMut::freeze).map(Body::from));
		};
		let (mut parts, body) = req.with_limited_body().into_parts();

		let header = match parts.extract::<TypedHeader<AuthHeader>>().await {
			Ok(TypedHeader(Authorization(header))) => Ok(header),
			Err(e) => Err(e.into_response()),
		}?;
		if let Some(dest) = header.destination {
			// Check somehow
		};

		let signatures = BTreeMap::from([(
			header.key.to_string(),
			CanonicalJsonValue::String(header.sig),
		)]);
		let signatures = BTreeMap::from([(
			header.origin.to_string(),
			CanonicalJsonValue::Object(signatures),
		)]);
		// Check somehow

		let body = axum::body::to_bytes(body, usize::MAX).await.unwrap();

		let (path_args, value) = (
			Path::<PathArgs>::from_request_parts(&mut parts, &()).await.unwrap(),
			serde_json::from_slice::<CanonicalJsonValue>(&body).unwrap(),
		);
		let http_request = http::Request::from_parts(parts, body);

		let body = T::try_from_http_request(http_request, &path_args).unwrap();

		Ok(Ruma { body })
	}
}
