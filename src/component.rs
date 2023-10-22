pub mod config;
pub mod site;
pub mod db;

#[tokio::main]
pub async fn start() {
	site::start().await;
}
