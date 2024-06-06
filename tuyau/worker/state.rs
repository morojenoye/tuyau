pub trait QueryExecutor {}

pub struct Executor<'a, T: QueryExecutor> {
	pub(super) query_executor: &'a T,
}
