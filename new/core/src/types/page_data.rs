use time::Date;
use time::macros::format_description;

use crate::core::*;
use crate::config::*;
use crate::types::*;
use crate::utils;
use crate::colours::*;
use crate::macros::*;

use std::collections::HashMap;
use std::path::PathBuf;


#[derive(Clone, Debug, Default, PartialEq, Eq, serde::Serialize)]
pub struct PageData
{
	/// Location of the original `.md` file this page represents.
	pub filepath: PathBuf,
	
	/// Location of the `+page.svx` file to render this page to.
	pub destination: PathBuf,
	
	pub flags: Strings,

	pub title: Option<String>,
	pub description: Option<String>,

	pub heading: Option<String>,
	pub caption: Option<String>,

	pub tags: Vec<String>,

	pub release_date: Option<Date>,
	pub last_updated: Option<Date>,

	#[serde(skip_serializing)]
	pub cleanse: Vec<CleanseOperation>,

	pub other: HashMap<String, Strings>,
}

impl PageData
{
	pub fn init(
		filepath: PathBuf,
		flags: Strings,
		mut fields: HashMap<String, Strings>,
		config: &SquarkupConfig,
	) -> SquarkResult<Self>
	{
		let mut errs = vec![];

		let mut destination = PathBuf::new();

		if let Some(dest) = Self::take1(&mut fields, "destination", "dest") {
			destination = config.out.folder.join(utils::rel_path(&dest));

			if !destination.starts_with(&config.paths.root) {
				errs.push(SquarkError::Unrecoverable {
					msg: str!(slash!("cannot export a file to {}", destination)),
					hint: fmt!("a file's destination directory must remain under the root directory of your project"),
					debug: vec![
						str!(slash!("your project's root directory is {}", config.paths.root))
					],
				});
			}
		}
		else {
			errs.push(SquarkError::Unrecoverable {
				msg: fmt!("missing field: {W}dest"),
				hint: fmt!("active pages must specify where they should be rendered to"),
				debug: vec![],
			});
		}
		
		let heading      = Self::take1(&mut fields, "heading", "head");
		let title        = Self::take1(&mut fields, "title", "title").or_else(|| heading.clone());
		
		// TODO better fallbacks
		let caption      = Self::take1(&mut fields, "caption", "capt");
		let description  = Self::take1(&mut fields, "description", "desc").or_else(|| caption.clone());

		let tags         = Self::take(&mut fields, "tags", "tags").unwrap_or(vec![]);

		let release_date = Self::take1(&mut fields, "release-date", "date")
			.map(|raw| Self::try_parse_date(&raw)).flatten();

		let last_updated = Self::take1(&mut fields, "last-updated", "update")
			.map(|raw| Self::try_parse_date(&raw)).flatten();

		let mut cleanse = vec![];

		for raw in Self::take(&mut fields, "cleanse", "clean").unwrap_or(vec![]) {
			match raw.clone().try_into()
			{
				Ok(value) => cleanse.push(value),
				Err(_) => {
					errs.push(SquarkError::Recoverable {
						msg: fmt!("invalid value for {W}cleanse{R}: {W}{raw}"),
						hint: fmt!("valid values are {W}angles{G}, {W}braces{G}, {W}comments{G}, {W}line-breaks"),
						debug: vec![],
					});
				}
			};
		}

		if errs.is_empty() {
			Ok(Self {
				filepath,
				flags,
				destination,
				title, description,
				heading, caption,
				tags,
				release_date, last_updated,
				cleanse,
				other: fields,
			})
		} else {
			Err(SquarkError::Multiple { errs })
		}
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

	fn try_parse_date(date: &str) -> Option<Date>
	{
		Date::parse(&date, &format_description!("[year] [month repr:long] [day]"))
			.or_else(|_| Date::parse(&date, &format_description!("[year] [month repr:short] [day]")))
			.ok()
	}
}
