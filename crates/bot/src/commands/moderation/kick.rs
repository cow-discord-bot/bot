use poise::CreateReply;
use serenity::all::User;

use crate::utils::dm_notifier_utils::send_mod_action_reason_dm;
use crate::{Context, Error};

/// Kick a guild member
#[poise::command(prefix_command, slash_command, guild_only)]
pub async fn kick(
	ctx: Context<'_>,
	#[description = "User to kick"] user: User,
	#[rest]
	#[description = "Reason"]
	reason: Option<String>,
) -> Result<(), Error> {
	ctx.defer().await?;

	// todo: check for config moderator role

	let guild_id = ctx
		.guild_id()
		.ok_or("This command can only be used in a guild")?;
	let reason_text = reason.as_deref().unwrap_or("No reason provided");

	let kick_result = guild_id
		.kick_with_reason(&ctx.serenity_context().http, user.id, reason_text)
		.await
		.err()
		.map(|e| format!("❌ Failed to kick user: {}\n", e));

	let mut response: String;
	if let Some(kick_result) = kick_result {
		response = kick_result;
	} else {
		response = format!("✅ Muted {}.\n", user.name);
		match send_mod_action_reason_dm(ctx, &user, "kicked", reason_text).await {
			| Ok(()) => response.push_str("✅ DM sent successfully."),
			| Err(_) => response.push_str("❌ Could not send DM."),
		}
	}

	ctx.send(
		CreateReply::default()
			.content(response)
			.ephemeral(ctx.prefix() == "/"),
	)
	.await?;

	Ok(())
}
