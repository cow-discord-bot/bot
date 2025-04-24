use std::env;

use poise::serenity_prelude::Context;
use serenity::all::{ChannelId, Color, CreateEmbed, CreateMessage, Event, RawEventHandler};

use crate::utils::mention::Mentionable;

pub struct Handler;

fn get_channel_id(env_key: &str) -> Option<ChannelId> {
	let id_str = env::var(env_key).ok()?;
	let id = id_str.parse().ok()?;
	Some(ChannelId::new(id))
}

#[serenity::async_trait]
impl RawEventHandler for Handler {
	async fn raw_event(
		&self,
		ctx: Context,
		new_event: Event,
	) {
		use serenity::model::event::Event::*;

		let (channel, title, color, fields) = match new_event {
			| MessageCreate(ref e) => {
				let Some(channel) = get_channel_id("MESSAGE_LOGS_CHANNEL_ID") else {
					return;
				};
				let user = &e.message.author;

				// todo dont hardcode
				if (user.id.to_string() == "1242367482346606633") {
					return;
				}

				let content = &e.message.content;
				(
					channel,
					"Message Sent",
					Color::BLURPLE,
					vec![
						("User", user.mention().to_string(), true),
						("Content", content.clone(), false),
					],
				)
			},
			| GuildBanAdd(event) => {
				let Some(channel) = get_channel_id("MODERATION_LOGS_CHANNEL_ID") else {
					return;
				};
				(
					channel,
					"User Banned",
					Color::RED,
					vec![
						("Guild", event.guild_id.to_string(), true),
						("User", event.user.mention().to_string(), true),
					],
				)
			},
			| _ => return,
		};

		let embed = CreateEmbed::new()
			.title(title)
			.colour(color)
			.fields(fields)
			.timestamp(serenity::model::Timestamp::now());

		let _ = channel
			.send_message(&ctx.http, CreateMessage::new().embed(embed))
			.await;
	}
}
