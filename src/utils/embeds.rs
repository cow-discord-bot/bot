use serenity::all::{Color, CreateEmbed};

pub fn create_error_embed(description: &str) -> CreateEmbed {
	CreateEmbed::default()
		.title("Error")
		.description(description)
		.color(Color::RED)
}
