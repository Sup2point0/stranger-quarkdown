use crate::core::*;
use crate::config::*;
use crate::types::*;
use crate::utils;
use crate::colours::*;
use crate::macros::*;

use path_clean::PathClean;
use time::Date;
use time::macros::format_description;

use std::collections::{ HashMap };
use std::path::{ PathBuf };


#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct PageData
{
	/// The stable unique identifier for the page.
	pub shard: String,

	/// The location of the original `.md` file this page represents.
	pub filepath: PathBuf,
	
	/// The folder to render this page's `+page.svx` and `+page.ts` to.
	pub destination: PathBuf,
	
	pub flags: Strings,

	pub title: Option<String>,
	pub description: Option<String>,

	pub heading: Option<String>,
	pub caption: Option<String>,

	pub tags: Vec<String>,

	pub release_date: Option<Date>,
	pub last_updated: Option<Date>,

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
		let mut errs = SquarkError::multiple();

		let mut destination = PathBuf::new();

		if let Some(dest) = Self::take1(&mut fields, "destination", "dest") {
			destination = config.out.folder.join(utils::to_rel(&dest)).clean();

			if config.errors.strict && !destination.starts_with(&config.paths.root) {
				errs.push(SquarkError::Unrecoverable {
					msg: str!(slash!("cannot export a file to: {}", destination)),
					hint: fmt!("a file's destination directory must remain under the root directory of your project"),
					debug: vec![
						str!(slash!("your project's root directory is: {}", config.paths.root)),
						str!(slash!("in file: {}", filepath)),
					],
				});
			}
		}
		else {
			errs.push(SquarkError::Unrecoverable {
				msg: fmt!("missing field: {W}dest"),
				hint: fmt!("active pages must specify where they should be rendered to"),
				debug: vec![
					str!(slash!("in file: {}", filepath)),
				],
			});
		}
		
		let heading      = Self::take1(&mut fields, "heading", "head");
		let title        = Self::take1(&mut fields, "title", "title").or_else(|| heading.clone());
		
		// TODO better fallbacks
		let caption      = Self::take1(&mut fields, "caption", "capt");
		let description  = Self::take1(&mut fields, "description", "desc").or_else(|| caption.clone());

		let tags         = Self::take(&mut fields, "tags", "tags").unwrap_or(vec![]);

		let release_date = Self::take1(&mut fields, "release-date", "date")
			.and_then(|raw| Self::try_parse_date(&raw));

		let last_updated = Self::take1(&mut fields, "last-updated", "update")
			.and_then(|raw| Self::try_parse_date(&raw));

		let mut cleanse = vec![];

		for raw in Self::take(&mut fields, "cleanse", "clean").unwrap_or(vec![]) {
			match raw.clone().try_into()
			{
				Ok(value) => cleanse.push(value),
				Err(_) => {
					errs.push(SquarkError::Recoverable {
						msg: fmt!("invalid value for {W}cleanse{R}: {W}{raw}"),
						hint: fmt!("valid values are {W}angles{G}, {W}braces{G}, {W}comments{G}, {W}line-breaks"),
						debug: vec![
							str!(slash!("in file: {}", filepath)),
						],
					});
				}
			}
		}

		errs.or_else(||
			Self {
				shard: utils::display_rel(&filepath, &config.paths.root),
				filepath,
				flags,
				destination,
				title, description,
				heading, caption,
				tags,
				release_date, last_updated,
				cleanse,
				other: fields,
			}
		)
	}

	fn take(fields: &mut HashMap<String, Strings>, long: &'static str, short: &'static str) -> Option<Vec<String>>
	{
		fields.remove(short)
			.or_else(|| fields.remove(long))
			.map(|t| t.into_vec())
	}

	fn take1(fields: &mut HashMap<String, Strings>, long: &'static str, short: &'static str) -> Option<String>
	{
		fields.remove(short)
			.or_else(|| fields.remove(long))?
			.into_iter().next()
	}

	fn try_parse_date(date: &str) -> Option<Date>
	{
		Date::parse(date, &format_description!("[year] [month repr:long] [day]"))
			.or_else(|_| Date::parse(date, &format_description!("[year] [month repr:short] [day]")))
			.ok()
	}
}

impl PageData
{
	#[must_use]
	pub fn serialise(self, config: &SquarkupConfig) -> SerialisedPageData
	{
		SerialisedPageData {
			filepath:     utils::display_rel(self.filepath, &config.paths.root),
			destination:  utils::display_rel(self.destination, &config.out.folder),
			flags:        self.flags,
			title:        self.title,
			description:  self.description,
			heading:      self.heading,
			caption:      self.caption,
			tags:         self.tags,
			release_date: self.release_date,
			last_updated: self.last_updated,
			other:        self.other,
		}
	}
}


#[derive(serde::Serialize)]
pub struct SerialisedPageData
{
	pub filepath: String,
	pub destination: String,
	pub flags: Strings,
	pub title: Option<String>,
	pub description: Option<String>,
	pub heading: Option<String>,
	pub caption: Option<String>,
	pub tags: Vec<String>,
	pub release_date: Option<Date>,
	pub last_updated: Option<Date>,
	pub other: HashMap<String, Strings>,
}

macro_rules! impl_field_repr
{
	($field:ident => $both:literal) => {
		impl_field_repr!($field => $both, $both);
	};
	($field:ident => $full:literal, $short:literal) =>
	{
		pub fn $field(&self, short: bool) -> &str {
			if short {$short} else {$full}
		}
	};
}

impl SerialisedPageData
{
	impl_field_repr!(filepath     => "filepath",     "path");
	impl_field_repr!(destination  => "destination",  "dest");
	impl_field_repr!(flags        => "flags");
	impl_field_repr!(title        => "title");
	impl_field_repr!(description  => "description",  "desc");
	impl_field_repr!(heading      => "heading",      "head");
	impl_field_repr!(caption      => "caption",      "capt");
	impl_field_repr!(tags         => "tags");
	impl_field_repr!(release_date => "release_date", "date");
	impl_field_repr!(last_updated => "last_updated", "update");
	impl_field_repr!(other        => "other",        "other");
}
