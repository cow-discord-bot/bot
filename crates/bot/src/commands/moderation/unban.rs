use poise::CreateReply;
use serenity::all::User;

use crate::{Context, Error};

/// Unban a guild member
#[poise::command(prefix_command, slash_command, guild_only)]
pub async fn unban(
	ctx: Context<'_>,
	#[description = "User to unban"] user: User,
) -> Result<(), Error> {
	ctx.defer().await?;

	// todo: check for config admin role

	let guild_id = ctx
		.guild_id()
		.ok_or("This command can only be used in a guild")?;

	let response = match guild_id.unban(&ctx.serenity_context().http, user.id).await {
		| Ok(_) => format!("✅ Unbanned {}.", user.name),
		| Err(e) => format!("❌ Failed to unban user: {}", e),
	};

	ctx.send(
		CreateReply::default()
			.content(response)
			.ephemeral(ctx.prefix() == "/"),
	)
	.await?;

	Ok(())
}
