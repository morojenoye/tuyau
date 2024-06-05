use ruma::ServerName;

use crate::{models::ServerKeys, MyResult};

pub trait Query {
	fn get_server_keys(server: &ServerName) -> MyResult<ServerKeys>;
}

pub struct Executor<'a, T: Query> {
	inner: &'a T,
}
