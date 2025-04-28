use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::post;
use axum::{Json, Router};
use serde::Serialize;

pub fn test() -> Router { Router::new().route("/health", post(handle_request)) }

#[derive(Serialize)]
struct Response {
	success: bool,
}

async fn handle_request() -> impl IntoResponse {
	(StatusCode::OK, Json(Response { success: true }))
}
