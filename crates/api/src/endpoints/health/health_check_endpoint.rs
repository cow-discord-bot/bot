use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{Json, Router};
use serde::Serialize;

pub fn health_check() -> Router { Router::new().route("/health", get(handle_request)) }

#[derive(Serialize)]
struct Response {
	success: bool,
}

async fn handle_request() -> impl IntoResponse {
	(StatusCode::OK, Json(Response { success: true }))
}
