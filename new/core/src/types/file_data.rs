use crate::{
	types::*,
	utils::macros::*,
};

use std::collections::HashMap;


#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct FileData
{
	pub flags: Strings,
	pub dest: String,

	pub title: Option<String>,
	pub description: Option<String>,

	pub heading: Option<String>,
	pub caption: Option<String>,
}

impl FileData
{
	pub fn init(
		flags: Strings,
		fields: HashMap<String, Strings>,
		config: &SquarkupConfig,
	) -> Result<Self, FileError>
	{
		let dest = Self::get(&fields, "dest")
			.ok_or_else(|| FileError::MissingField { field: str!("dest") })?;
		
		let heading     = Self::get(&fields, "head");
		let title       = Self::get(&fields, "title").or_else(|| heading.clone());
		
		// TODO better fallbacks
		let caption     = Self::get(&fields, "capt");
		let description = Self::get(&fields, "desc").or_else(|| caption.clone());

		Ok(Self {
			flags,
			dest: dest.to_string(),
			title,
			description,
			heading,     
			caption,
		})
	}

	fn get(fields: &HashMap<String, Strings>, field: &'static str) -> Option<String>
	{
		fields
			.get(field)?
			.iter().next()
			.map(|s| s.clone())
	}
}
