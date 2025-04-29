use axum::Router;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get;

pub fn health_check() -> Router { Router::new().route("/health", get(handle_request)) }

async fn handle_request() -> impl IntoResponse { (StatusCode::OK, "hi") }
