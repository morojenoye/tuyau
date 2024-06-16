use crate::Ref;

pub trait QueryExecutor {}

#[derive(Clone)]
pub struct Executor<T: QueryExecutor> {
	pub(super) state: Ref<T>,
}
