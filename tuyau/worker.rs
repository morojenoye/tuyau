use ruma::{RoomId, UserId};

pub mod keyserver;
pub mod state;
pub mod timeline;

pub trait Query: state::Query + timeline::Query {}

pub struct Executor<'a, T: Query> {
	state: state::Executor<'a, T>,
	timeline: timeline::Executor<'a, T>,
}

impl<'a, T: Query> Executor<'a, T> {
	pub fn new(room_id: &RoomId, user_id: &UserId) {}
	pub fn get() {}
}
