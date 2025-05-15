use poise::CreateReply;
use serenity::all::User;

use crate::utils::dm_notifier_utils::send_mod_action_reason_dm;
use crate::{Context, Error};

/// Ban a guild member and delete all messages
#[poise::command(prefix_command, slash_command, guild_only)]
pub async fn dban(
	ctx: Context<'_>,
	#[description = "User to ban"] user: User,
	#[rest]
	#[description = "Reason"]
	reason: Option<String>,
) -> Result<(), Error> {
	ctx.defer().await?;

	// todo: check for config admin role

	let guild_id = ctx
		.guild_id()
		.ok_or("This command can only be used in a guild")?;
	let reason_text = reason.as_deref().unwrap_or("No reason provided");

	let ban_result = guild_id
		.ban_with_reason(&ctx.serenity_context().http, user.id, 0, reason_text)
		.await
		.err()
		.map(|e| format!("❌ Failed to ban user: {}\n", e));

	let mut response: String;
	if let Some(ban_result) = ban_result {
		response = ban_result;
	} else {
		response = format!("✅ Banned {}.\n", user.name);
		match send_mod_action_reason_dm(ctx, &user, "banned", reason_text).await {
			| Ok(()) => response.push_str("✅ DM sent successfully."),
			| Err(_) => response.push_str("❌ Could not send DM."),
		}
	}

	// todo: purge rest of their messages, waiting on /purge backend

	ctx.send(
		CreateReply::default()
			.content(response)
			.ephemeral(ctx.prefix() == "/"),
	)
	.await?;

	Ok(())
}
