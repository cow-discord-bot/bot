use std::env::var;

use poise::serenity_prelude::Context;
use serenity::all::{ChannelId, Color, CreateEmbed, CreateMessage, Event, RawEventHandler};

use crate::utils::mention::Mentionable;

pub struct Handler;

fn get_channel_id(
	guild_id: u64,
	key: &str,
) -> Option<ChannelId> {
	let db = sled::open("data/guild_settings/log_channels").ok()?;
	let tree = db.open_tree(guild_id.to_string()).ok()?;
	let ivec = tree.get(key.as_bytes()).ok()??;
	let id = u64::from_be_bytes(ivec.as_ref().try_into().ok()?);
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
			| MessageCreate(event) => {
				let Some(guild_id) = event.message.guild_id else {
					return;
				};

				let Some(channel) = get_channel_id(guild_id.get(), "MESSAGE_SENT_CHANNEL_ID")
				else {
					return;
				};

				let user = event.message.author;

				// prevent creating logs of log creation messages
				if user.id.to_string()
					== var("BOT_ID").unwrap_or("set your user id env var bro".to_string())
				{
					return;
				}

				// todo: attach the emssage attachments, or better yet, uplaod them to server and attach link
				(channel, "Message Sent", Color::BLURPLE, vec![
					("User", user.mention().to_string(), true),
					("Content", event.message.content.clone(), false),
				])
			},
			| GuildBanAdd(event) => {
				let Some(channel) = get_channel_id(event.guild_id.get(), "BAN_CHANNEL_ID") else {
					return;
				};

				// todo: get last banned user in the guild in order to get the banner info, banner info would only work if they use this bots ban command, discord ban command i dont think we can use for the info unless we use the audit log event
				(channel, "User Banned", Color::RED, vec![(
					"Banned User",
					event.user.mention().to_string(),
					true,
				)])
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
