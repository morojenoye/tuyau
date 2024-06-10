use ruma::{RoomAliasId, UserId};

#[derive(Clone)]
pub struct Setup<'a> {
	pub room_id: &'a RoomAliasId,
	pub user_id: &'a UserId,
}
