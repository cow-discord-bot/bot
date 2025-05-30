//! This file is autogenerated by build.rs, do not edit.
#![cfg_attr(rustfmt, rustfmt_skip)]

#[path = "$guild_id/mod.rs"]
pub mod guild_id;
pub mod health;

pub use axum::Router;

pub fn add_endpoints(app: Router) -> Router {
	app.merge(guild_id::settings::log_channel::set_log_channel::set_log_channel()).merge(health::health::health())
}