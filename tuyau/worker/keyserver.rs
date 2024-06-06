use async_trait::async_trait;

use ruma::{api::federation::discovery::ServerSigningKeys, ServerName};

use crate::MyResult;

#[async_trait]
pub trait QueryExecutor {
	async fn get_server_keys(&self, server: &ServerName) -> MyResult<ServerSigningKeys>;
}

pub struct Executor<'a, T: QueryExecutor> {
	pub(super) query_executor: &'a T,
}

impl<'a, T: QueryExecutor> Executor<'a, T> {
	pub async fn get_server_keys(&self, server: &ServerName) -> MyResult<ServerSigningKeys> {
		self.query_executor.get_server_keys(server).await
	}
}
