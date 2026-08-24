use crate::{
	types::*,
	utils::log,
	utils::macros::*,
};

use std::collections::HashMap;


#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct FileData
{
	// TODO link to source file
	
	pub flags: Strings,
	pub destination: String,

	pub title: Option<String>,
	pub description: Option<String>,

	pub heading: Option<String>,
	pub caption: Option<String>,

	pub tags: Vec<String>,

	// FIXME dates
	pub release_date: Option<String>,
	pub last_updated: Option<String>,

	pub cleanse: Vec<CleanseOperation>,

	pub other: HashMap<String, Strings>,
}

impl FileData
{
	pub fn init(
		flags: Strings,
		mut fields: HashMap<String, Strings>,
		config: &SquarkupConfig,
	) -> Result<Self, FileError>
	{
		let dest = Self::take1(&mut fields, "destination", "dest")
			.ok_or_else(|| FileError::MissingField { field: str!("dest") })?;
		
		let heading      = Self::take1(&mut fields, "heading", "head");
		let title        = Self::take1(&mut fields, "title", "title").or_else(|| heading.clone());
		
		// TODO better fallbacks
		let caption      = Self::take1(&mut fields, "caption", "capt");
		let description  = Self::take1(&mut fields, "description", "desc").or_else(|| caption.clone());

		let tags         = Self::take(&mut fields, "tags", "tags").unwrap_or(vec![]);

		let release_date = Self::take1(&mut fields, "release-date", "date");
		let last_updated = Self::take1(&mut fields, "last-updated", "update");

		let mut cleanse = vec![];

		for raw in Self::take(&mut fields, "cleanse", "clean").unwrap_or(vec![]) {
			let Ok(value) = raw.clone().try_into() else {
				let err = FileError::InvalidValue {
					field: str!("cleanse"),
					value: raw,
				};

				match config.errors.on_error {
					ErrorAction::KILL => return Err(err),
					ErrorAction::WARN => { log::bad!(err); continue; },
				}
			};

			cleanse.push(value);
		}

		Ok(Self {
			flags,
			destination: dest.to_string(),
			title, description,
			heading, caption,
			tags,
			release_date, last_updated,
			cleanse,
			other: fields,
		})
	}

	fn take(fields: &mut HashMap<String, Strings>, long: &'static str, short: &'static str) -> Option<Vec<String>>
	{
		fields.remove(short)
			.or_else(|| fields.remove(long))
			.map(|t| t.to_vec())
	}

	fn take1(fields: &mut HashMap<String, Strings>, long: &'static str, short: &'static str) -> Option<String>
	{
		fields.remove(short)
			.or_else(|| fields.remove(long))?
			.into_iter().next()
	}
}
