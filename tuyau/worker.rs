use ruma::{RoomId, ServerName, UserId};

pub mod keyserver;
pub mod state;
pub mod timeline;

pub trait QueryExecutor:
	keyserver::QueryExecutor + state::QueryExecutor + timeline::QueryExecutor + Sync
{
}

pub struct Executor<'a, T: QueryExecutor> {
	pub keyserver: keyserver::Executor<'a, T>,
	pub state: state::Executor<'a, T>,
	pub timeline: timeline::Executor<'a, T>,
	pub server_name: &'a ServerName,
}

impl<'a, T: QueryExecutor> Executor<'a, T> {
	pub fn new(query_executor: &'a T, room_id: &'a RoomId, user_id: &UserId) -> Self {
		Self {
			keyserver: keyserver::Executor { query_executor },
			state: state::Executor { query_executor },
			timeline: timeline::Executor { query_executor },
			server_name: room_id.server_name().unwrap(),
		}
	}
	pub fn get() {}
}
