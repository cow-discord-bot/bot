use serenity::all::User;

pub trait Mentionable {
	fn mention(&self) -> String;
}

impl Mentionable for User {
	fn mention(&self) -> String { format!("<@{}>", self.id) }
}
