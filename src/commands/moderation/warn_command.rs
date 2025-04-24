use poise::CreateReply;
use poise::serenity_prelude::ComponentInteractionCollector;
use rusqlite::{Connection, params};
use serde::{Deserialize, Serialize};
use serenity::all::{
	ButtonStyle,
	CreateActionRow,
	CreateButton,
	CreateEmbed,
	CreateEmbedFooter,
	CreateInteractionResponse,
	CreateInteractionResponseMessage,
	CreateMessage,
	GuildId,
	User,
};

use crate::utils::dates::format_timestamp_ddmmyyyy;
use crate::{Context, Error};

const WARNS_PER_PAGE: usize = 10;

/// Warn a guild member
#[poise::command(prefix_command, slash_command, subcommands("list"), guild_only)]
pub async fn warn(
	ctx: Context<'_>,
	#[description = "User to warn"] user: User,
	#[rest]
	#[description = "Reason"]
	reason: Option<String>,
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
	#[description = "User to show warnings for"] user: User,
) -> Result<(), Error> {
	ctx.defer().await?;

	let guild_id = ctx
		.guild_id()
		.ok_or("This command can only be used in a guild.")?;

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

	if warns.is_empty() {
		ctx.say(format!("{} has no warnings.", user.name)).await?;
		return Ok(());
	}

	let total_warns = warns.len();
	let total_pages = (total_warns + WARNS_PER_PAGE - 1) / WARNS_PER_PAGE;
	let mut current_page = 0;

	let create_embed_page = |page: usize| -> CreateEmbed {
		let start = page * WARNS_PER_PAGE;
		let end = (start + WARNS_PER_PAGE).min(total_warns);

		let description = warns[start..end]
			.iter()
			.map(|warn| {
				format!(
					"**{}** - {}",
					format_timestamp_ddmmyyyy(&warn.timestamp),
					warn.reason
				)
			})
			.collect::<Vec<_>>()
			.join("\n");

		let footer_text = format!(
			"Page {}/{} • Total Warnings: {}",
			page + 1,
			total_pages,
			total_warns
		);

		CreateEmbed::default()
			.title(format!("Warnings for {}", user.name))
			.description(description)
			.footer(CreateEmbedFooter::new(footer_text))
	};

	let create_components = |page: usize| {
		vec![CreateActionRow::Buttons(vec![
			CreateButton::new("first")
				.label("◀◀")
				.style(ButtonStyle::Primary)
				.disabled(page == 0),
			CreateButton::new("prev")
				.label("◀")
				.style(ButtonStyle::Secondary)
				.disabled(page == 0),
			CreateButton::new("next")
				.label("▶")
				.style(ButtonStyle::Secondary)
				.disabled(page + 1 >= total_pages),
			CreateButton::new("last")
				.label("▶▶")
				.style(ButtonStyle::Primary)
				.disabled(page + 1 >= total_pages),
		])]
	};

	let response = ctx
		.send(
			CreateReply::default()
				.embed(create_embed_page(current_page))
				.components(create_components(current_page)),
		)
		.await?;

	while let Some(interaction) = ComponentInteractionCollector::new(ctx.serenity_context())
		.message_id(response.message().await?.id)
		.author_id(ctx.author().id)
		.timeout(std::time::Duration::from_secs(60 * 2))
		.await
	{
		let action = interaction.data.custom_id.as_str();
		match action {
			| "first" => current_page = 0,
			| "prev" => {
				if current_page > 0 {
					current_page -= 1;
				}
			},
			| "next" => {
				if current_page + 1 < total_pages {
					current_page += 1;
				}
			},
			| "last" => current_page = total_pages - 1,
			| _ => {},
		}

		interaction
			.create_response(
				ctx.serenity_context(),
				CreateInteractionResponse::UpdateMessage(
					CreateInteractionResponseMessage::new()
						.embed(create_embed_page(current_page))
						.components(create_components(current_page)),
				),
			)
			.await?;
	}

	response
		.edit(
			ctx,
			poise::CreateReply::default()
				.embed(create_embed_page(current_page))
				.components(vec![]),
		)
		.await?;

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
