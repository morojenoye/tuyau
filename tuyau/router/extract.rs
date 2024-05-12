use async_trait::async_trait;
use axum::{
	body::Body,
	extract::{FromRequest, FromRequestParts, Path},
	http::Request,
	RequestExt,
};
use ruma::{
	api::{AuthScheme, IncomingRequest},
	CanonicalJsonValue,
};

type PathArgs = Vec<String>;

pub struct Ruma<T> {
	pub body: T,
}

#[async_trait]
impl<T, S> FromRequest<S> for Ruma<T>
where
	T: IncomingRequest,
{
	type Rejection = ();

	async fn from_request(req: Request<Body>, _: &S) -> Result<Self, Self::Rejection> {
		let AuthScheme::ServerSignatures = T::METADATA.authentication else {
			return Err(());
		};
		let (mut parts, body) = req.with_limited_body().into_parts();

		let value = serde_json::from_slice::<CanonicalJsonValue>(
			&axum::body::to_bytes(body, usize::MAX).await.unwrap(),
		)
		.unwrap();
		let path_args = Path::<PathArgs>::from_request_parts(&mut parts, &()).await.unwrap();

		let body = serde_json::to_vec(&value).unwrap();

		let http_request = http::Request::from_parts(parts, body);

		let body = T::try_from_http_request(http_request, &path_args).unwrap();

		Ok(Ruma { body })
	}
}
