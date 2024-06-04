use {crate::Global, std::collections::BTreeMap};

use async_trait::async_trait;
use axum::{
	body,
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
	CanonicalJsonValue, OwnedServerName,
};
use {bytes::BytesMut, http::StatusCode};

type AuthHeader = Authorization<XMatrix>;
type PathArgs = Vec<String>;

pub struct Ruma<T> {
	pub body: T,
}

fn make_internal_server_error() -> Response {
	StatusCode::INTERNAL_SERVER_ERROR.into_response()
}

fn make_unauthorized_error() -> Response {
	let error_body = ErrorBody::Standard {
		kind: ErrorKind::Unauthorized,
		message: String::new(),
	};
	let error = error_body.into_error(StatusCode::UNAUTHORIZED);

	let Ok(response) = error.try_into_http_response::<BytesMut>() else {
		return StatusCode::INTERNAL_SERVER_ERROR.into_response();
	};
	return response.map(BytesMut::freeze).map(Body::from);
}

#[async_trait]
impl<T> FromRequest<Global> for Ruma<T>
where
	T: IncomingRequest,
{
	type Rejection = Response;

	async fn from_request(req: Request<Body>, global: &Global) -> Result<Self, Self::Rejection> {
		let AuthScheme::ServerSignatures = T::METADATA.authentication else {
			return Err(make_unauthorized_error());
		};
		let (mut parts, body) = req.with_limited_body().into_parts();

		let header = match parts.extract::<TypedHeader<AuthHeader>>().await {
			Ok(TypedHeader(Authorization(header))) => Ok(header),
			Err(e) => Err(e.into_response()),
		}?;

		let check = |dest: OwnedServerName| {
			let differ = dest != global.server_name;
			differ.then(make_unauthorized_error)
		};
		match header.destination.map(check).flatten() {
			Some(response) => return Err(response),
			None => (),
		}
		let mut request_map = BTreeMap::new();

		let keys = ["method", "uri", "destination", "origin"];

		request_map.insert(keys[0].to_string(), parts.method.to_string().into());
		request_map.insert(keys[1].to_string(), parts.uri.to_string().into());

		let body = body::to_bytes(body, usize::MAX).await;
		let body = body.map_err(|_| make_internal_server_error())?;

		let (path_args, value) = (
			Path::<PathArgs>::from_request_parts(&mut parts, &()).await.unwrap(),
			serde_json::from_slice::<CanonicalJsonValue>(&body).ok(),
		);
		value.map(|v| request_map.insert("content".to_string(), v));

		let signatures = BTreeMap::from([(
			header.key.to_string(),
			CanonicalJsonValue::String(header.sig),
		)]);
		let signatures = CanonicalJsonValue::Object(BTreeMap::from([(
			header.origin.to_string(),
			CanonicalJsonValue::Object(signatures),
		)]));
		request_map.insert("signatures".to_string(), signatures);

		let server_name = global.server_name.to_string().into();
		let origin = header.origin.to_string().into();

		request_map.insert(keys[2].to_string(), server_name);
		request_map.insert(keys[3].to_string(), origin);

		let http_request = http::Request::from_parts(parts, body);

		let body = T::try_from_http_request(http_request, &path_args).unwrap();

		Ok(Ruma { body })
	}
}
