pub trait Query {}

pub struct Executor<'a, T: Query> {
	inner: &'a T,
}
