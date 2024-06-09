use ruma::{RoomId, UserId};

#[derive(Clone)]
pub struct Setup<'a> {
	pub room_id: &'a RoomId,
	pub user_id: &'a UserId,
}
