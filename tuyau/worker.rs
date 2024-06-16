use {anyhow::anyhow, std::marker};

use ruma::{OwnedRoomAliasId, OwnedRoomId, OwnedServerName, OwnedUserId, RoomId};

use crate::{Maybe, MyResult, Ref};

pub mod keyserver;
pub mod state;
pub mod timeline;

pub struct SetupBundle {
	pub alias: OwnedRoomAliasId,
	pub admin: OwnedUserId,
	pub ident: OwnedRoomId,
}

pub trait QueryExecutor:
	keyserver::QueryExecutor
	+ state::QueryExecutor
	+ timeline::QueryExecutor
	+ marker::Send
	+ marker::Sync
{
	async fn new(&self, setup: SetupBundle) -> MyResult<()>;
	async fn get(&self) -> MyResult<Maybe<SetupBundle>>;
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
	pub async fn new(state: Ref<T>, alias: OwnedRoomAliasId, admin: OwnedUserId) -> MyResult<Self> {
		let maybe_setup: Option<SetupBundle> = {
			// =============================================================
			QueryExecutor::get(state.as_ref()).await?
		};
		let ident: OwnedRoomId = if let Some(setup) = maybe_setup {
			// =============================================================
			if setup.alias != alias || setup.admin != admin {
				return Err(anyhow!("aboba"));
			}
			// =============================================================
			setup.ident
		} else {
			// =============================================================
			let ident: OwnedRoomId = RoomId::new(alias.server_name());
			// =============================================================
			let setup: SetupBundle = SetupBundle {
				alias: alias.clone(),
				admin: admin.clone(),
				ident: ident.clone(),
			};
			QueryExecutor::new(state.as_ref(), setup).await?;
			// =============================================================
			ident
		};
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
