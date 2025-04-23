use poise::CreateReply;
use serenity::all::{CreateMessage, User};

use crate::utils::guild_logs::{LogType, log_event};
use crate::{Context, Error};

#[poise::command(prefix_command, slash_command)]
pub async fn ban(
	ctx: Context<'_>,
	#[description = "User to ban"] user: User,
	#[description = "Reason"] reason: Option<String>,
	#[description = "Delete messages?"] delete_messages: Option<bool>,
) -> Result<(), Error> {
	ctx.defer().await?;

	let guild_id = ctx
		.guild_id()
		.ok_or("This command can only be used in a guild")?;
	let reason_text = reason.as_deref().unwrap_or("No reason provided");
	let delete_days = if delete_messages.unwrap_or(false) {
		7
	} else {
		0
	};

	let mut dm_result = send_ban_reason_dm(ctx, &user, reason_text).await;

	let ban_result = guild_id
		.ban_with_reason(
			&ctx.serenity_context().http,
			user.id,
			delete_days,
			reason_text,
		)
		.await;

	let mut response = String::new();

	match ban_result {
		| Ok(_) => {
			response.push_str(&format!("✅ Banned {}.\n", user.tag()));
		},
		| Err(e) => {
			response.push_str(&format!("❌ Failed to ban user: {}\n", e));
			dm_result = Ok(());
		},
	}

	match dm_result {
		| Ok(()) => response.push_str("✅ DM sent successfully."),
		| Err(_) => response.push_str("❌ Could not send DM."),
	}

	ctx.send(
		CreateReply::default()
			.content(response)
			.ephemeral(ctx.prefix() == "/"),
	)
	.await?;

	Ok(())
}

async fn send_ban_reason_dm(
	ctx: Context<'_>,
	user: &User,
	reason: &str,
) -> Result<(), Error> {
	if let Some(guild_id) = ctx.guild_id() {
		let guild_name = guild_id.name(ctx.cache()).unwrap_or("a server".into());
		user.dm(
			ctx.serenity_context(),
			CreateMessage::new().content(format!(
				"You have been banned from **{}**.\n**Reason**: {}",
				guild_name, reason
			)),
		)
		.await?;
	}

	log_event(&ctx, LogType::Ban, "test".to_string()).await?;
	Ok(())
}
