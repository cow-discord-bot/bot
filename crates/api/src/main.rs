mod endpoints;

#[tokio::main]
async fn main() {
	tracing_subscriber::fmt::init();
	let app = endpoints::add_endpoints(axum::Router::new());

	let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
	axum::serve(listener, app).await.unwrap();
}
