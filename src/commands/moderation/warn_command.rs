use poise::CreateReply;
use rusqlite::{Connection, params};
use serde::{Deserialize, Serialize};
use serenity::all::{CreateMessage, GuildId, User};

use crate::utils::dates::format_timestamp_ddmmyyyy;
use crate::{Context, Error};

/// Warn a guild member
#[poise::command(prefix_command, slash_command, subcommands("list"), guild_only)]
pub async fn warn(
	ctx: Context<'_>,
	#[description = "User to warn"] user: User,
	#[description = "Reason"] reason: Option<String>,
) -> Result<(), Error> {
	ctx.defer().await?;

	let guild_id = ctx
		.guild_id()
		.ok_or("This command can only be used in a guild.")?;

	let reason_text = reason.as_deref().unwrap_or("No reason provided");

	let response = match send_warn_reason_dm(ctx, &user, reason_text).await {
		| Ok(()) => &format!("✅ warned {}.", user.name),
		| Err(_) => "❌ Could not send DM.",
	};

	add_warn(user, guild_id, &reason_text).await?;

	ctx.send(
		CreateReply::default()
			.content(response)
			.ephemeral(ctx.prefix() == "/"),
	)
	.await?;

	Ok(())
}

// todo: use embed, add pagination, at the bottom show the page number of total pages and the total warns amount
/// Show all warns of a guild member
#[poise::command(
	prefix_command,
	slash_command,
	invoke_on_edit,
	reuse_response,
	guild_only
)]
pub async fn list(
	ctx: Context<'_>,
	#[description = "User to warn"] user: User,
) -> Result<(), Error> {
	let Some(guild_id) = ctx.guild_id() else {
		ctx.say("This command can only be used in a server.")
			.await?;
		return Ok(());
	};

	let table_name = format!("guild_{}", guild_id);

	let conn = Connection::open("src/data/user_warns.db")?;

	let user_id = user.id.to_string();
	let warns: Vec<Warning> = conn
		.prepare(&format!(
			"SELECT warns FROM {} WHERE user_id = ?1",
			table_name
		))?
		.query_row(params![user_id], |row| {
			let warns_str: String = row.get(0)?;
			serde_json::from_str(&warns_str).map_err(|e| {
				rusqlite::Error::FromSqlConversionFailure(
					0,
					rusqlite::types::Type::Text,
					Box::new(e),
				)
			})
		})
		.unwrap_or_else(|_| vec![]);

	let response = if warns.is_empty() {
		format!("{} has no warnings.", user.name)
	} else {
		let lines: Vec<String> = warns
			.iter()
			.enumerate()
			.map(|(_i, warn)| {
				format!(
					"{} - {}",
					format_timestamp_ddmmyyyy(&warn.timestamp),
					warn.reason,
				)
			})
			.collect();
		lines.join("\n")
	};

	ctx.send(CreateReply::default().content(response)).await?;

	Ok(())
}

async fn send_warn_reason_dm(
	ctx: Context<'_>,
	user: &User,
	reason: &str,
) -> Result<(), Error> {
	if let Some(guild_id) = ctx.guild_id() {
		let guild_name = guild_id.name(ctx.cache()).unwrap_or("Unkown server".into());
		user.dm(
			ctx.serenity_context(),
			CreateMessage::new().content(format!(
				"**{}**: You have been warned.\n**Reason**: {}",
				guild_name, reason
			)),
		)
		.await?;
	}
	Ok(())
}

#[derive(Serialize, Deserialize)]
struct Warning {
	reason:    String,
	timestamp: String,
}

pub async fn add_warn(
	user: User,
	guild_id: GuildId,
	reason: &str,
) -> Result<(), Error> {
	let conn = Connection::open("src/data/user_warns.db")?;

	let table_name = format!("guild_{}", guild_id);

	conn.execute(
		&format!(
			"CREATE TABLE IF NOT EXISTS {} (
                user_id TEXT PRIMARY KEY,
                warns TEXT NOT NULL
            )",
			table_name
		),
		[],
	)?;

	let mut stmt = conn.prepare(&format!(
		"SELECT warns FROM {} WHERE user_id = ?1",
		table_name
	))?;

	let user_id = user.id.to_string();
	let mut warnings: Vec<Warning> = stmt
		.query_row(params![user_id], |row| {
			let warns_str: String = row.get(0)?;
			serde_json::from_str(&warns_str).map_err(|e| {
				rusqlite::Error::FromSqlConversionFailure(
					0,
					rusqlite::types::Type::Text,
					Box::new(e),
				)
			})
		})
		.unwrap_or_else(|_| vec![]);

	warnings.push(Warning {
		reason:    reason.to_string(),
		timestamp: chrono::Utc::now().to_rfc3339(),
	});

	let warns_json = serde_json::to_string(&warnings)?;

	conn.execute(
		&format!(
			"INSERT INTO {} (user_id, warns)
             VALUES (?1, ?2)
             ON CONFLICT(user_id) DO UPDATE SET warns = excluded.warns",
			table_name
		),
		params![user_id, warns_json],
	)?;

	Ok(())
}
