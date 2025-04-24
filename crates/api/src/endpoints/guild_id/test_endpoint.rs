use axum::Router;
use axum::extract::{Json, Path};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::post;
use serde::Deserialize;

pub fn test() -> Router { Router::new().route("/{guild_id}/test", post(handle_test)) }

#[derive(Deserialize)]
struct TestRequest {
	message: String,
}

async fn handle_test(
	Path(guild_id): Path<String>,
	Json(payload): Json<TestRequest>,
) -> impl IntoResponse {
	let reply = format!("Guild {} said: {}", guild_id, payload.message);

	(StatusCode::OK, reply)
}
