use serenity::all::{Color, CreateEmbed};

use crate::types::Error;
use crate::utils::tag_utils::TagError;

pub trait ToEmbed {
	fn to_embed(self) -> CreateEmbed;
}

impl ToEmbed for Error {
	fn to_embed(self) -> CreateEmbed {
		CreateEmbed::default()
			.title("Error")
			.description(self.to_string())
			.color(Color::RED)
	}
}

impl ToEmbed for TagError {
	fn to_embed(self) -> CreateEmbed {
		CreateEmbed::default()
			.title("Error")
			.description(self.to_string())
			.color(Color::RED)
	}
}
