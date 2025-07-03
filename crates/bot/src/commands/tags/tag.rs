use poise::CreateReply;
use serenity::all::{CreateEmbed, CreateMessage};
use tokio::time::Instant;

use crate::utils::embeds::create_error_embed;
use crate::utils::tag_utils::get_data_and_id;
use crate::{Context, Error};

#[poise::command(
	prefix_command,
	slash_command,
	subcommands("create", "edit", "delete", "list", "preview", "raw", "alias"),
	invoke_on_edit,
	reuse_response,
	guild_only
)]
pub async fn tag(
	ctx: Context<'_>,
	#[description = "Tag name"] name: String,
) -> Result<(), Error> {
	let referenced_message = match &ctx {
		| Context::Prefix(prefix_ctx) => prefix_ctx.msg.message_reference.clone(),
		| _ => None,
	};

	let (data, id) = get_data_and_id(ctx).await?;

	match data.tag_db.get_tag(&name, id).await {
		| Ok(v) => {
			let mut message = CreateMessage::default().content(v);

			if let Some(msg_ref) = referenced_message {
				message = message.reference_message(msg_ref);
			}

			ctx.channel_id()
				.send_message(ctx.serenity_context(), message)
				.await?;
		},
		| Err(e) => {
			ctx.send(CreateReply::default().embed(create_error_embed(&e.to_string()))).await?;
		}
	}

	Ok(())
}

/// Create a new tag
#[poise::command(
	prefix_command,
	slash_command,
	invoke_on_edit,
	reuse_response,
	guild_only,
	aliases("add")
)]
async fn create(
	ctx: Context<'_>,
	#[description = "Tag name"] name: String,
	#[description = "Tag content"]
	#[rest]
	content: String,
) -> Result<(), Error> {
	let (data, id) = get_data_and_id(ctx).await?;

	match data.tag_db.create_tag(&name, &content, id).await {
		| Ok(()) => {
			ctx.send(CreateReply::default().content(format!("✅ Created tag `{}`", name)))
				.await?
		},
		| Err(e) => {
			ctx.send(CreateReply::default().embed(create_error_embed(&e.to_string())))
				.await?
		},
	};

	Ok(())
}

/// Delete an existing tag
#[poise::command(
	prefix_command,
	slash_command,
	invoke_on_edit,
	reuse_response,
	guild_only
)]
async fn delete(
	ctx: Context<'_>,
	#[description = "Tag name"] name: String,
) -> Result<(), Error> {
	let (data, id) = get_data_and_id(ctx).await?;

	match data.tag_db.delete_tag(&name, id).await {
		| Ok(name) => {
			ctx.send(CreateReply::default().content(format!("✅ Deleted tag `{}`", name)))
				.await?
		},
		| Err(e) => {
			ctx.send(CreateReply::default().embed(create_error_embed(&e.to_string())))
				.await?
		},
	};

	Ok(())
}

/// Edit an existing tag
#[poise::command(
	prefix_command,
	slash_command,
	invoke_on_edit,
	reuse_response,
	guild_only
)]
async fn edit(
	ctx: Context<'_>,
	#[description = "Tag name"] name: String,
	#[description = "New content"]
	#[rest]
	content: String,
) -> Result<(), Error> {
	let (data, id) = get_data_and_id(ctx).await?;

	match data.tag_db.edit_tag(&name, &content, id).await {
		| Ok(name) => {
			ctx.send(CreateReply::default().content(format!("✅ Updated tag `{}`", name)))
				.await?
		},
		| Err(e) => {
			ctx.send(CreateReply::default().embed(create_error_embed(&e.to_string())))
				.await?
		},
	};

	Ok(())
}

/// List all tags for this server
#[poise::command(
	prefix_command,
	slash_command,
	invoke_on_edit,
	reuse_response,
	guild_only
)]
async fn list(ctx: Context<'_>) -> Result<(), Error> {
	let (data, id) = get_data_and_id(ctx).await?;

	match data.tag_db.get_all_tags(id).await {
		| Ok(tags) => {
			let formatted_tags = if tags.is_empty() {
				"No tags found. Try creating a tag with `/tag create`".to_string()
			} else {
				tags.join(", ")
			};

			ctx.send(
				CreateReply::default()
					.embed(
						CreateEmbed::default()
							.title("All Tags")
							.description(formatted_tags),
					)
					.ephemeral(true),
			)
			.await?
		},
		| Err(e) => {
			ctx.send(
				CreateReply::default()
					.embed(create_error_embed(&e.to_string()))
					.ephemeral(true),
			)
			.await?
		},
	};
	Ok(())
}

/// Privately preview a tag
#[poise::command(slash_command, invoke_on_edit, reuse_response, guild_only)]
async fn preview(
	ctx: Context<'_>,
	#[description = "Tag name"] name: String,
) -> Result<(), Error> {
	let (data, id) = get_data_and_id(ctx).await?;

	match data.tag_db.get_tag(&name, id).await {
		| Ok(content) => {
			ctx.send(CreateReply::default().content(content).ephemeral(true))
				.await?
		},
		| Err(e) => {
			ctx.send(CreateReply::default().embed(create_error_embed(&e.to_string())))
				.await?
		},
	};

	Ok(())
}

/// View a tag in raw text
#[poise::command(prefix_command, slash_command, invoke_on_edit, reuse_response)]
async fn raw(
	ctx: Context<'_>,
	#[description = "Tag name"] name: String,
) -> Result<(), Error> {
	let (data, id) = get_data_and_id(ctx).await?;

	match data.tag_db.get_tag(&name, id).await {
		| Ok(content) => {
			ctx.send(CreateReply::default().content(content.replace("`", "\\`")
			.replace("*", "\\*")
			.replace("_", "\\_")
			.replace("~", "\\~")
			.replace("#", "\\#")
			.replace("<", "\\<")
			.replace(">", "\\>")
			.replace("|", "\\|")))
				.await?
		},
		| Err(e) => {
			ctx.send(CreateReply::default().embed(create_error_embed(&e.to_string())))
				.await?
		},
	};

	Ok(())
}

/// Create an alias for an existing tag
#[poise::command(prefix_command, slash_command, invoke_on_edit, reuse_response)]
async fn alias(
	ctx: Context<'_>,
	#[description = "Tag name"] name: String,
	#[description = "Tag alias"] alias: String,
) -> Result<(), Error> {
	let (data, id) = get_data_and_id(ctx).await?;

	match data.tag_db.get_tag(&name, id).await {
		| Ok(content) => {
			match data.tag_db.create_tag(&alias, &content, id).await {
				| Ok(()) => {
					ctx.send(
						CreateReply::default().content(format!("✅ Created tag alias `{}`", alias)),
					)
					.await?
				},
				| Err(e) => {
					ctx.send(CreateReply::default().embed(create_error_embed(&e.to_string())))
						.await?
				},
			}
		},
		| Err(e) => {
			ctx.send(CreateReply::default().embed(create_error_embed(&e.to_string())))
				.await?
		},
	};

	Ok(())
}
