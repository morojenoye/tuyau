use async_trait::async_trait;
use bytes::BytesMut;
use std::collections::BTreeMap;

use axum::{
	body::{self, Body},
	extract::{FromRequest, FromRequestParts, Path},
	http::Request,
	response::{IntoResponse, Response as Reply},
	RequestExt, RequestPartsExt,
};
use axum_extra::{headers::Authorization, TypedHeader};
use http::StatusCode;
use ruma::{
	api::{
		client::error::{ErrorBody, ErrorKind},
		federation::discovery::VerifyKey,
		AuthScheme, IncomingRequest, OutgoingResponse,
	},
	serde::Base64,
	server_util::authorization::XMatrix,
	signatures::verify_json,
	CanonicalJsonValue, OwnedServerName, OwnedServerSigningKeyId,
};

use crate::worker::{Executor, QueryExecutor};

type Result<Ty> = std::result::Result<Ty, Reply>;
type AuthHeader = Authorization<XMatrix>;
type PathArgs = Vec<String>;

pub struct Ruma<T> {
	pub body: T,
}

fn make_internal_server_error() -> Reply {
	StatusCode::INTERNAL_SERVER_ERROR.into_response()
}

fn make_unauthorized_error() -> Reply {
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
impl<'a, R, T> FromRequest<Executor<'a, T>> for Ruma<R>
where
	R: IncomingRequest,
	T: QueryExecutor,
{
	type Rejection = Reply;

	async fn from_request(req: Request<Body>, ctx: &Executor<'a, T>) -> Result<Self> {
		// =================================================================

		let AuthScheme::ServerSignatures = R::METADATA.authentication else {
			return Err(make_unauthorized_error());
		};
		let (mut parts, body) = req.with_limited_body().into_parts();

		// =================================================================

		let header: _ = match parts.extract::<TypedHeader<AuthHeader>>().await {
			Ok(TypedHeader(Authorization(header))) => Ok(header),
			Err(e) => Err(e.into_response()),
		}?;
		let check = |dest: OwnedServerName| {
			let differ = dest != ctx.server_name;
			differ.then(make_unauthorized_error)
		};
		match header.destination.map(check).flatten() {
			Some(response) => return Err(response),
			None => (),
		}

		// =================================================================

		let keys: [&str; 4] = ["method", "uri", "destination", "origin"];
		let mut request_map: _ = BTreeMap::new();

		request_map.insert(keys[0].to_string(), parts.method.to_string().into());
		request_map.insert(keys[1].to_string(), parts.uri.to_string().into());

		// =================================================================

		let body = body::to_bytes(body, usize::MAX).await;
		let body = body.map_err(|_| make_internal_server_error())?;

		let (path_args, value) = (
			Path::<PathArgs>::from_request_parts(&mut parts, &()).await.unwrap(),
			serde_json::from_slice::<CanonicalJsonValue>(&body).ok(),
		);
		value.map(|value: _| request_map.insert("content".to_string(), value));

		// =================================================================

		let signatures: BTreeMap<String, CanonicalJsonValue> = BTreeMap::from([(
			header.key.to_string(),
			CanonicalJsonValue::String(header.sig),
		)]);
		let signatures: BTreeMap<String, CanonicalJsonValue> = BTreeMap::from([(
			header.origin.to_string(),
			CanonicalJsonValue::Object(signatures),
		)]);
		let signatures: _ = CanonicalJsonValue::Object(signatures);

		request_map.insert("signatures".to_string(), signatures);

		// =================================================================

		let server_name = ctx.server_name.to_string().into();
		let origin = header.origin.to_string().into();

		request_map.insert(keys[2].to_string(), server_name);
		request_map.insert(keys[3].to_string(), origin);

		// =================================================================

		let (server_keys, mut p_keys_map) = (
			ctx.keyserver.get_server_keys(&header.origin).await,
			BTreeMap::new(),
		);
		let server_keys: _ = server_keys.map_err(|_| make_internal_server_error())?;
		let iter: _ = server_keys.verify_keys.into_iter();

		let v: BTreeMap<String, Base64> = iter.map(fmt).collect();
		p_keys_map.insert(header.origin.to_string(), v);

		// =================================================================

		if let Err(_) = verify_json(&p_keys_map, &request_map) {
			return Err(make_internal_server_error());
		}
		let http_request = http::Request::from_parts(parts, body);

		let body = R::try_from_http_request(http_request, &path_args).unwrap();

		Ok(Ruma { body })
	}
}

type Input = (OwnedServerSigningKeyId, VerifyKey);
type Value = (String, Base64);

fn fmt(payloads: Input) -> Value {
	let (k, v) = payloads;
	(k.to_string(), v.key)
}
