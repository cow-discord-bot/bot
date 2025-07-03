use std::fmt;

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use sled::Db;
use strsim::jaro_winkler;

use crate::Data;
use crate::types::Context;

pub struct TagDb {
	db: Db,
}

impl TagDb {
	pub fn new() -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
		Ok(TagDb {
			db: sled::open("data/tags")?,
		})
	}

	pub async fn create_tag(
		&self,
		name: &str,
		content: &str,
		guild_id: u64,
	) -> Result<(), TagError> {
		let tree = self.db.open_tree(guild_id.to_string())?;

		if let Some(_value) = tree.get(name.as_bytes())? {
			return Err(TagError::AlreadyExists(name.to_string()));
		}

		tree.insert(name.as_bytes(), content.as_bytes())?;
		Ok(())
	}

	pub async fn delete_tag(
		&self,
		name: &str,
		guild_id: u64,
	) -> Result<String, TagError> {
		let tree = self.db.open_tree(guild_id.to_string())?;
		let fixed_name = self.fix_typos(name, guild_id).await?;

		if let Some(_value) = tree.get(fixed_name.as_bytes())? {
			tree.remove(fixed_name.as_bytes())?;
			return Ok(fixed_name);
		}

		return Err(TagError::DoesntExist(name.to_string()));
	}

	pub async fn edit_tag(
		&self,
		name: &str,
		content: &str,
		guild_id: u64,
	) -> Result<String, TagError> {
		let tree = self.db.open_tree(guild_id.to_string())?;
		let fixed_name = self.fix_typos(name, guild_id).await?;

		if let Some(_value) = tree.get(fixed_name.as_bytes())? {
			tree.insert(fixed_name.as_bytes(), content.as_bytes())?;
			return Ok(fixed_name);
		}

		Err(TagError::DoesntExist(fixed_name.to_string()))
	}

	async fn get_tag_exact(
		&self,
		name: &str,
		guild_id: u64,
	) -> Result<String, TagError> {
		let tree = self.db.open_tree(guild_id.to_string())?;

		if let Some(value) = tree.get(name.as_bytes())? {
			return Ok(str::from_utf8(&value)?.to_owned());
		}

		Err(TagError::DoesntExist(name.to_string()))
	}

	pub async fn get_tag(
		&self,
		name: &str,
		guild_id: u64,
	) -> Result<String, TagError> {
		let fixed_name = self.fix_typos(name, guild_id).await?;

		return Ok(self.get_tag_exact(&fixed_name, guild_id).await?);
	}

	pub async fn get_all_tags(
		&self,
		guild_id: u64,
	) -> Result<Vec<String>, TagError> {
		let tree = self.db.open_tree(guild_id.to_string())?;

		let mut tags = Vec::<String>::new();

		for item in tree.iter() {
			let (key, _value) = item?;
			tags.push(str::from_utf8(&key)?.to_owned());
		}

		Ok(tags)
	}

	/// returns input value if no fix is found
	async fn fix_typos(
		&self,
		name: &str,
		guild_id: u64,
	) -> Result<String, TagError> {
		let all_tags = self.get_all_tags(guild_id).await?;
		if all_tags.is_empty() {
			return Ok(name.to_owned());
		}

		let best_match = all_tags.par_iter().max_by(|a, b| {
			jaro_winkler(name, a)
				.partial_cmp(&jaro_winkler(name, b))
				.unwrap_or(std::cmp::Ordering::Equal)
		});

		if let Some(best) = best_match {
			let similarity = jaro_winkler(name, best);
			if similarity > 0.80 {
				return Ok(best.to_owned());
			}
		}

		Ok(name.to_owned())
	}
}

#[derive(Debug)]
pub enum TagError {
	DoesntExist(String),
	AlreadyExists(String),
	NotGuild(),
	Serenity(String),
	Database(String),
	Utf8Error(String),
}

impl std::error::Error for TagError {}
impl From<std::str::Utf8Error> for TagError {
	fn from(err: std::str::Utf8Error) -> TagError {
		TagError::Utf8Error(err.to_string())
	}
}

impl From<serenity::Error> for TagError {
	fn from(err: serenity::Error) -> TagError {
		TagError::Serenity(err.to_string())
	}
}

impl From<sled::Error> for TagError {
	fn from(err: sled::Error) -> TagError {
		TagError::Database(err.to_string())
	}
}

impl std::fmt::Display for TagError {
	fn fmt(
		&self,
		f: &mut fmt::Formatter<'_>,
	) -> fmt::Result {
		match self {
			| TagError::DoesntExist(tag) => write!(f, "❌ Tag `{}` doesn't exist!", tag),
			| TagError::AlreadyExists(tag) => write!(f, "❌ Tag `{}` already exists!", tag),
			| TagError::NotGuild() => write!(f, "❌ This command can only be run in servers."),
			| TagError::Serenity(e) => write!(f, "❌ {}", e),
			| TagError::Database(e) => write!(f, "❌ {}", e),
			| TagError::Utf8Error(e) => write!(f, "❌ {}", e),
		}
	}
}

pub async fn get_data_and_id(ctx: Context<'_>) -> Result<(&Data, u64), TagError> {
	let data = ctx.data();

	let id = match ctx.guild_id() {
		| Some(id) => id.get(),
		| None => {
			return Err(TagError::NotGuild());
		},
	};

	Ok((data, id))
}
