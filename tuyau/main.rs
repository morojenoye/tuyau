mod models;
mod router;
mod setups;
mod worker;

pub type MyResult<T> = anyhow::Result<T>;

#[tokio::main]
async fn main() {}
