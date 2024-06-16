use std::collections::BTreeMap;

use async_trait::async_trait;
use axum::{
	body::{self, Body},
	extract::{FromRequest, FromRequestParts, Path},
	http::Request,
	RequestExt, RequestPartsExt,
};
use axum_extra::{headers::Authorization, TypedHeader};

use ruma::{
	api::federation::discovery::VerifyKey,
	api::{client::error::ErrorKind, AuthScheme, IncomingRequest},
	serde::Base64,
	server_util::authorization::XMatrix,
	signatures::verify_json,
	CanonicalJsonValue, OwnedServerName, OwnedServerSigningKeyId,
};

use crate::{
	router::reply::MApiError,
	worker::{keyserver, Executor, QueryExecutor},
};

type MyResult<Ty> = std::result::Result<Ty, ErrReply>;
type ErrReply = MApiError<ErrorKind>;

type AuthHeader = Authorization<XMatrix>;
type PathArgs = Vec<String>;

pub struct MApi<T> {
	pub body: T,
}

#[async_trait]
impl<R, T> FromRequest<Executor<T>> for MApi<R>
where
	R: IncomingRequest,
	T: QueryExecutor,
{
	type Rejection = MApiError<ErrorKind>;

	async fn from_request(req: Request<Body>, ctx: &Executor<T>) -> MyResult<Self> {
		// =================================================================

		let AuthScheme::ServerSignatures = R::METADATA.authentication else {
			return Err(MApiError(ErrorKind::forbidden()));
		};
		let (mut parts, body) = req.with_limited_body().into_parts();

		// =================================================================

		let header = match parts.extract::<TypedHeader<AuthHeader>>().await {
			Ok(TypedHeader(Authorization(header))) => Ok(header),
			Err(_) => Err(MApiError(ErrorKind::Unauthorized)),
		}?;
		let check = |dest: OwnedServerName| {
			let differ: bool = dest != ctx.server_name();
			differ.then_some(MApiError(ErrorKind::Unauthorized))
		};
		match header.destination.map(check).flatten() {
			Some(response) => return Err(response),
			None => (),
		}

		// =================================================================

		let keys: [&str; 4] = ["method", "uri", "destination", "origin"];
		let mut request_map = BTreeMap::new();

		request_map.insert(keys[0].to_string(), parts.method.to_string().into());
		request_map.insert(keys[1].to_string(), parts.uri.to_string().into());

		// =================================================================

		let body = body::to_bytes(body, usize::MAX).await;
		let body = body.map_err(|_| MApiError(ErrorKind::TooLarge))?;

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
		let signatures = CanonicalJsonValue::Object(signatures);

		request_map.insert("signatures".to_string(), signatures);

		// =================================================================

		let server_name = ctx.server_name().to_string().into();
		let origin = header.origin.to_string().into();

		request_map.insert(keys[2].to_string(), server_name);
		request_map.insert(keys[3].to_string(), origin);

		// =================================================================

		let keyserver: &keyserver::Executor<T> = &ctx.keyserver;
		let server: &OwnedServerName = &header.origin;

		let iter: _ = match keyserver.get(server).await {
			Ok(server_keys) => server_keys.verify_keys.into_iter(),
			Err(_) => Err(MApiError(ErrorKind::Unauthorized))?,
		};
		let mut p_key_map = BTreeMap::new();

		let v: BTreeMap<_, Base64> = iter.map(fmt).collect();
		p_key_map.insert(header.origin.to_string(), v);

		// =================================================================

		let http_request: _ = match verify_json(&p_key_map, &request_map) {
			Err(_) => Err(MApiError(ErrorKind::Unauthorized))?,
			Ok(()) => http::Request::from_parts(parts, body),
		};
		match R::try_from_http_request(http_request, &path_args) {
			Err(_) => Err(MApiError(ErrorKind::BadJson)),
			Ok(dt) => Ok(MApi { body: dt }),
		}
	}
}

type Input = (OwnedServerSigningKeyId, VerifyKey);
type Value = (String, Base64);

fn fmt(payloads: Input) -> Value {
	let (k, v) = payloads;
	(k.to_string(), v.key)
}
