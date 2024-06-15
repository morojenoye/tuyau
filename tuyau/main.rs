use {sea_orm::Database, tokio::net::TcpListener};

use axum::{routing::get, Router};
use ruma::{owned_room_alias_id, owned_user_id};

use crate::{models::DefaultQueryExecutor, router::get_room_information_route, worker::Executor};

mod models;
mod router;
mod worker;

pub type MyResult<T> = anyhow::Result<T>;
pub type Ref<T> = std::sync::Arc<T>;

#[tokio::main]
async fn main() -> MyResult<()> {
	let inner = Database::connect("sqlite:tuyau.db3?mode=rwc").await?;

	let (query_executor, room_id, user_id) = (
		Ref::new(DefaultQueryExecutor::new(inner).await?),
		owned_room_alias_id!("#stokejo:stokejo.com"),
		owned_user_id!("@mekosko:projectyo.network"),
	);
	let state = Executor::new(query_executor, room_id, user_id)?;

	let app = Router::new()
		.route(
			"/_matrix/federation/v1/query/directory",
			get(get_room_information_route),
		)
		.with_state(state);

	let tcp = TcpListener::bind(":::2727").await?;

	axum::serve(tcp, app).await?;

	return Ok(());
}
