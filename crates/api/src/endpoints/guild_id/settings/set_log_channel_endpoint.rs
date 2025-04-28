use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::post;
use axum::{Json, Router};
use serde::{Deserialize, Serialize};

pub fn set_log_channel() -> Router {
	Router::new().route("/{guild_id}/settings/log-channel", post(handle_request))
}

#[derive(Deserialize, Debug)]
struct SetLogChannel {
	log_type:   String,
	channel_id: u64,
}

#[derive(Serialize)]
struct Response {
	success: bool,
	#[serde(skip_serializing_if = "Option::is_none")]
	reason:  Option<String>,
}

// todo: have some enum for all possible log types for checking
// todo: check if channel id is a valid id, and maybe if the guil has that channel id
// todo: auth and check if person making request has permission to do so
async fn handle_request(
	Path(guild_id): Path<String>,
	Json(payload): Json<SetLogChannel>,
) -> impl IntoResponse {
	match set_channel_id(&payload.log_type, payload.channel_id, guild_id).await {
		| Ok(()) => {
			let response = Response {
				success: true,
				reason:  None,
			};
			(StatusCode::OK, Json(response))
		},
		| Err(e) => {
			let response = Response {
				success: false,
				reason:  Some(e.to_string()),
			};
			(StatusCode::BAD_REQUEST, Json(response))
		},
	}
}

async fn set_channel_id(
	key: &str,
	channel_id: u64,
	guild_id: String,
) -> Result<(), sled::Error> {
	println!("Current directory: {:?}", std::env::current_dir().unwrap());
	println!("fn called");
	let db = sled::open("data/guild_settings/log_channels")?;
	println!("db opened");
	let tree = db.open_tree(guild_id)?;
	println!("tree opened");
	tree.insert(key.as_bytes(), &channel_id.to_be_bytes())?;
	println!("key inserted");
	tree.flush()?;
	println!("flushed");
	Ok(())
}
