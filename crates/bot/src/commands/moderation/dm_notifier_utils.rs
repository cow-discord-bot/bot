use serenity::all::{CreateMessage, User};

use crate::{Context, Error};

pub async fn send_mod_action_reason_dm(
	ctx: Context<'_>,
	user: &User,
	r#type: &str,
	reason: &str,
) -> Result<(), Error> {
	if let Some(guild_id) = ctx.guild_id() {
		let guild_name = guild_id
			.name(ctx.cache())
			.unwrap_or("Unknown server".into());
		user.dm(
			ctx.serenity_context(),
			CreateMessage::new().content(format!(
				"**{}**: You have been {}.\n**Reason**: {}",
				guild_name, r#type, reason
			)),
		)
		.await?;
	}
	Ok(())
}
