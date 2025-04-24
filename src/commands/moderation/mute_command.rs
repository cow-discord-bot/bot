use std::env::var;

use poise::CreateReply;
use serenity::all::{
	CreateMessage,
	EditChannel,
	PermissionOverwrite,
	PermissionOverwriteType,
	Permissions,
	RoleId,
	User,
};
use serenity::builder::EditRole;
use serenity::model::id::GuildId;
use serenity::prelude::*;

use crate::commands::moderation::dm_notifier_utils::send_mod_action_reason_dm;
use crate::{Context, Error};

/// Mute a guild member
#[poise::command(prefix_command, slash_command, aliases("timeout"), guild_only)]
pub async fn mute(
	ctx: Context<'_>,
	#[description = "User to mute"] user: User,
	#[rest]
	#[description = "Reason"]
	reason: Option<String>,
) -> Result<(), Error> {
	ctx.defer().await?;

	let reason_text = reason.as_deref().unwrap_or("No reason provided");

	let guild_id = ctx
		.guild_id()
		.ok_or("This command can only be used in a guild.")?;

	let muted_role_id = get_or_create_muted_role(&ctx, guild_id).await?;

	let member = guild_id
		.member(ctx.serenity_context(), user.id)
		.await
		.map_err(|e| format!("Failed to fetch member: {}", e))?;

	member
		.add_role(ctx.serenity_context(), muted_role_id)
		.await
		.map_err(|e| format!("Failed to assign Muted role: {}", e))?;

	let dm_result = send_mod_action_reason_dm(ctx, &user, "muted", reason_text).await;

	let mut response = format!("✅ Muted {}.\n", user.name);
	match dm_result {
		| Ok(()) => response.push_str("✅ DM sent successfully."),
		| Err(_) => response.push_str("❌ Could not send DM."),
	}

	match override_channel_perms(&ctx, guild_id, muted_role_id).await {
		| Ok(()) => {},
		| _ => response.push_str("❌ Could not update channel permissions."),
	}

	ctx.send(
		CreateReply::default()
			.content(response)
			.ephemeral(ctx.prefix() == "/"),
	)
	.await?;

	Ok(())
}

async fn get_or_create_muted_role(
	ctx: &Context<'_>,
	guild_id: GuildId,
) -> Result<RoleId, Box<dyn std::error::Error + Send + Sync>> {
	if let Ok(role_id_str) = var("MUTED_ROLE_ID") {
		if let Ok(role_id_num) = role_id_str.parse::<u64>() {
			return Ok(RoleId::new(role_id_num));
		}
	}

	let guild = guild_id
		.to_partial_guild(ctx.http())
		.await
		.map_err(|e| format!("Failed to fetch guild: {}", e))?;

	if let Some(role) = guild
		.roles
		.values()
		.find(|r| r.name.eq_ignore_ascii_case("Muted"))
	{
		let unwanted_permissions = Permissions::SEND_MESSAGES
			| Permissions::SPEAK
			| Permissions::SEND_TTS_MESSAGES
			| Permissions::ADD_REACTIONS;

		if role.permissions.intersects(unwanted_permissions) {
			let new_permissions = role.permissions - unwanted_permissions;
			guild_id
				.edit_role(
					ctx.http(),
					role.id,
					EditRole::new().permissions(new_permissions),
				)
				.await?;
		}

		return Ok(role.id);
	}

	let new_role = guild_id
		.create_role(
			&ctx.http(),
			EditRole::new()
				.name("Muted")
				.permissions(Permissions::empty()),
		)
		.await?
		.id;

	Ok(new_role)
}

// should run on channel update evnts too
pub async fn override_channel_perms(
	ctx: &Context<'_>,
	guild_id: GuildId,
	muted_role_id: RoleId,
) -> Result<(), Error> {
	let deny_permissions = Permissions::SEND_MESSAGES
		| Permissions::SPEAK
		| Permissions::ADD_REACTIONS
		| Permissions::SEND_MESSAGES_IN_THREADS;
	let channels = guild_id.channels(&ctx.http()).await?;

	for (channel_id, channel) in channels {
		let mut overwrites = channel.permission_overwrites.clone();
		let updated: bool;

		if let Some(pos) = overwrites
			.iter()
			.position(|po| po.kind == PermissionOverwriteType::Role(muted_role_id))
		{
			overwrites[pos].deny |= deny_permissions;
			overwrites[pos].allow &= !deny_permissions;
			updated = true;
		} else {
			overwrites.push(PermissionOverwrite {
				allow: Permissions::empty(),
				deny:  deny_permissions,
				kind:  PermissionOverwriteType::Role(muted_role_id),
			});
			updated = true;
		}

		if updated {
			channel_id
				.edit(ctx.http(), EditChannel::new().permissions(overwrites))
				.await?;
		}
	}

	Ok(())
}
