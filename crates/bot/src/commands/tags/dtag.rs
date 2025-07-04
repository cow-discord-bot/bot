use poise::CreateReply;
use serenity::all::CreateMessage;

use crate::utils::embeds::ToEmbed;
use crate::utils::tag_utils::get_data_and_id;
use crate::{Context, Error};

#[poise::command(prefix_command, guild_only)]
pub async fn dtag(
	ctx: Context<'_>,
	#[description = "Tag name"] name: String,
) -> Result<(), Error> {
	let msg = match ctx {
		| Context::Prefix(prefix_ctx) => Some(prefix_ctx.msg),
		| _ => None,
	};

	let (data, id) = get_data_and_id(ctx).await?;

		match data.tag_db.get_tag(&name, id).await {
		| Ok(v) => {
			let mut message = CreateMessage::default().content(v);

		if let Some(referenced_message) = msg.and_then(|m| m.message_reference.clone()) {
			message = message.reference_message(referenced_message);
		}

			ctx.channel_id()
				.send_message(ctx.serenity_context(), message)
				.await?;
		},
		| Err(e) => {
			ctx.send(CreateReply::default().embed(e.to_embed())).await?;
		}
	}

	if let Some(msg) = msg {
		msg.delete(ctx.serenity_context()).await?;
	}

	Ok(())
}
