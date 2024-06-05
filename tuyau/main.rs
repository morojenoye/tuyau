mod models;
mod router;
mod setups;
mod worker;

pub type MyResult<T> = anyhow::Result<T>;

pub struct Global {
	server_name: String,
}

#[tokio::main]
async fn main() {}
