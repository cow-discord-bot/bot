use serenity::all::{Role, RoleId, User, UserId};

pub trait Mentionable {
	fn mention(&self) -> String;
}

impl Mentionable for User {
	fn mention(&self) -> String { format!("<@{}>", self.id) }
}

impl Mentionable for UserId {
	fn mention(&self) -> String { format!("<@{}>", self) }
}

impl Mentionable for Role {
	fn mention(&self) -> String { format!("<@&{}>", self.id) }
}

impl Mentionable for RoleId {
	fn mention(&self) -> String { format!("<@&{}>", self) }
}
