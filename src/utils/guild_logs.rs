use std::env::var;

use serenity::all::{ChannelId, Color, CreateEmbed, CreateMessage};

use crate::{Context, Error};

#[derive(Debug)]
pub enum LogType {
	MessageSent,
	MessageDeleted,
	MessageEdited,
	ReactionAdded,
	ReactionRemoved,
	EmbedDeleted,
	AttachmentDeleted,
	VoiceJoined,
	VoiceLeft,
	UserJoined,
	UserLeft,
	Ban,
	Kick,
	Mute,
}

pub async fn log_event(
	ctx: &Context<'_>,
	log_type: LogType,
	content: impl Into<String>,
) -> Result<(), Error> {
	let message_log_channel = ChannelId::new(var("MESSAGE_LOGS_CHANNEL_ID")?.parse()?);
	let member_log_channel = ChannelId::new(var("MEMBER_LOGS_CHANNEL_ID")?.parse()?);
	let mod_log_channel = ChannelId::new(var("MODERATION_LOGS_CHANNEL_ID")?.parse()?);

	let (channel_id, title, color) = match log_type {
		| LogType::MessageSent => (message_log_channel, "Message Sent", Color::DARK_GREEN),
		| LogType::MessageDeleted => (message_log_channel, "Message Deleted", Color::RED),
		| LogType::MessageEdited => (message_log_channel, "Message Edited", Color::ORANGE),
		| LogType::ReactionAdded => (message_log_channel, "Reaction Added", Color::DARK_GREEN),
		| LogType::ReactionRemoved => (message_log_channel, "Reaction Removed", Color::DARK_ORANGE),
		| LogType::EmbedDeleted => (message_log_channel, "Embed Removed", Color::RED),
		| LogType::AttachmentDeleted => (message_log_channel, "Attachment Removed", Color::RED),

		| LogType::VoiceJoined => (
			message_log_channel,
			"User Joined Voice Channel",
			Color::DARK_GREEN,
		),
		| LogType::VoiceLeft => (
			message_log_channel,
			"User Left Voice Channel",
			Color::DARK_ORANGE,
		),

		| LogType::UserJoined => (member_log_channel, "User Joined Server", Color::DARK_GREEN),
		| LogType::UserLeft => (member_log_channel, "User Left Server", Color::DARK_RED),

		| LogType::Ban => (mod_log_channel, "User Banned", Color::RED),
		| LogType::Kick => (mod_log_channel, "User Kicked", Color::RED),
		| LogType::Mute => (mod_log_channel, "User Muted", Color::DARK_GREY),
	};

	let embed = CreateEmbed::new()
		.title(title)
		.description(content.into())
		.color(color)
		.timestamp(serenity::model::Timestamp::now());

	channel_id
		.send_message(&ctx.http(), CreateMessage::new().embed(embed))
		.await?;

	Ok(())
}
