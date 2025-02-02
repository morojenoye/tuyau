use ruma::{api::federation::discovery::ServerSigningKeys, ServerName};

use crate::{MyResult, Ref};

pub trait QueryExecutor {
	async fn get(&self, server: &ServerName) -> MyResult<ServerSigningKeys>;
}

#[derive(Clone)]
pub struct Executor<T: QueryExecutor> {
	pub(super) state: Ref<T>,
}

impl<T: QueryExecutor> Executor<T> {
	pub async fn get(&self, request: &ServerName) -> MyResult<ServerSigningKeys> {
		let s = ("https", request.host(), "/_matrix/key/v2/server");
		let s = format!("{}://{}:8448{}", s.0, s.1, s.2);
		Ok(reqwest::get(s).await?.json().await?)
	}
}
