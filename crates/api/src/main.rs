use std::env::var;

use tracing::{Level, info};

mod endpoints;

#[tokio::main]
async fn main() {
	tracing_subscriber::fmt()
		.with_max_level(Level::DEBUG)
		.init();

	let app = endpoints::add_endpoints(axum::Router::new());

	let listener = tokio::net::TcpListener::bind(format!(
		"0.0.0.0:{}",
		var("API_PORT").unwrap_or("3000".to_string())
	))
	.await
	.unwrap();

	info!("listening on {}", listener.local_addr().unwrap());
	axum::serve(listener, app).await.unwrap();
}
