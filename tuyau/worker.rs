use std::marker;

use ruma::{OwnedRoomAliasId, OwnedRoomId, OwnedServerName, OwnedUserId, RoomId};

use crate::{MyResult, Ref};

pub mod keyserver;
pub mod setup;
pub mod state;
pub mod timeline;

pub trait QueryExecutor:
	keyserver::QueryExecutor
	+ setup::QueryExecutor
	+ state::QueryExecutor
	+ timeline::QueryExecutor
	+ marker::Send
	+ marker::Sync
{
}

#[derive(Clone)]
pub struct Executor<T: QueryExecutor> {
	// =====================================================================
	pub keyserver: keyserver::Executor<T>,
	pub state: state::Executor<T>,
	pub timeline: timeline::Executor<T>,
	// =====================================================================
	pub alias: OwnedRoomAliasId,
	pub admin: OwnedUserId,
	// =====================================================================
	pub ident: OwnedRoomId,
}

impl<T: QueryExecutor> Executor<T> {
	pub fn new(state: Ref<T>, alias: OwnedRoomAliasId, admin: OwnedUserId) -> MyResult<Self> {
		// Check room_alias_id and user_id
		// Get room_id from db
		// If setup table is empty create new entry
		let ident = RoomId::new(alias.server_name());

		Ok(Self {
			// =============================================================
			keyserver: keyserver::Executor {
				state: state.clone(),
			},
			state: state::Executor {
				state: state.clone(),
			},
			timeline: timeline::Executor {
				state: state.clone(),
			},
			// =============================================================
			alias,
			admin,
			// =============================================================
			ident,
		})
	}

	pub fn server_name(&self) -> OwnedServerName {
		OwnedServerName::from(self.alias.server_name())
	}
}
