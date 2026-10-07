use crate::config::*;
use crate::types::*;
use crate::utils;
use crate::colours::*;
use crate::macros::*;

use path_clean::PathClean;
use path_macro::path;
use serde::ser::SerializeMap;
use time::Date;
use time::macros::format_description;

use std::collections::{ HashMap };
use std::path::{ Path, PathBuf };


/// The metadata provided for an active page, parsed from its charm squark.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct PageData
{
	/// The unique stable identifier for the page, normalised from `.filepath`.
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
	pub release_date_raw: Option<String>,
	pub last_update: Option<Date>,
	pub last_update_raw: Option<String>,

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
		let mut errs = SquarkError::multiple(slash!("initialising {}", filepath));

		let destination = {
			if let Some(raw) = Self::take_flat(&mut fields, "destination", "dest") {
				let dest = path!(config.out.folder / utils::to_rel(&raw.replace(' ', "-"))).clean();

				if config.errors.strict && !dest.starts_with(&config.paths.root) {
					errs.push(SquarkError::Unrecoverable {
						msg: slash!("cannot export a file to: {}", dest),
						hint: fmt!("output files must remain under your project root when {Y}errors.strict{G} is enabled"),
						debug: vec![
							slash!("your project's root directory is: {}", config.paths.root),
							slash!("in file: {}", filepath),
						],
					});
				}

				dest
			}
			else if config.errors.strict {
				errs.push(SquarkError::Unrecoverable {
					msg: fmt!("missing field: {W}dest{R}({W}ination{R})"),
					hint: fmt!("active pages must specify where they should be rendered to when {Y}errors.strict{G} is enabled"),
					debug: vec![
						fmt!("parsed fields: {GREY1}{fields:?}")
					],
				});
				PathBuf::new()
			}
			else {
				let filepath_rel = filepath.strip_prefix(&config.paths.root)
					.expect("source files are always under project root");

				let mut dest_rel = filepath_rel.with_extension("");

				if let Some(filename) = dest_rel.file_name()
					&& filename.eq_ignore_ascii_case("readme")
				{
					dest_rel = dest_rel.parent()
						.expect("source files always have a parent folder")
						.to_path_buf()
					;
				}

				path!(config.out.folder / dest_rel).clean()
			}
		};
		
		let heading      = Self::take_flat(&mut fields, "heading", "head");
		let title        = Self::take_flat(&mut fields, "title", "title").or_else(|| heading.clone());
		
		let caption      = Self::take_flat(&mut fields, "caption", "capt");
		let description  = Self::take_flat(&mut fields, "description", "desc").or_else(|| caption.clone());

		let tags         = Self::take(&mut fields, "tags", "tags").unwrap_or_default();

		let mut release_date = None;
		let mut release_date_raw = None;

		if let Some(raw) = Self::take_flat(&mut fields, "release_date", "date") {
			catch!(errs => {
				release_date = Some(Self::try_parse_date(&raw, "release date")?);
			});
			release_date_raw = Some(raw);
		}

		let mut last_update = release_date;
		let mut last_update_raw = release_date_raw.clone();

		if let Some(raw) = Self::take_flat(&mut fields, "last_update", "update") {
			catch!(errs => {
				last_update = Some(Self::try_parse_date(&raw, "last updated")?);
			});
			last_update_raw = Some(raw);
		}

		if config.errors.strict
		&& let Some(date) = release_date
		&& let Some(update) = last_update
		&& update < date
		{
			errs.push(SquarkError::Recoverable {
				msg: fmt!("you provided a {W}last updated{R} date earlier than the {W}release date"),
				hint: str!("you can’t update a page before you release it!"),
				debug: vec![
					fmt!("last updated = {}", last_update_raw.as_ref().unwrap()),
					fmt!("release date = {}", release_date_raw.as_ref().unwrap()),
				],
			});
		}

		let mut cleanse = vec![];

		for raw in Self::take(&mut fields, "cleanse", "clean").unwrap_or_default() {
			match raw.parse::<CleanseOperation>()
			{
				Ok(value) => cleanse.push(value),
				Err(_) => {
					errs.push(SquarkError::Recoverable {
						msg: fmt!("invalid value for {W}cleanse{R}: {W}{raw}"),
						hint: fmt!("valid values are {W}angles{G}, {W}braces{G}, {W}comments{G}, {W}line-breaks"),
						debug: vec![
							slash!("in file: {}", filepath),
						],
					});
				}
			}
		}

		errs.or_else(||
			Self {
				shard: Self::shard_for(&filepath, config),
				filepath,
				flags,
				destination,
				title, description,
				heading, caption,
				tags,
				release_date, release_date_raw,
				last_update, last_update_raw,
				cleanse,
				other: fields,
			}
		)
	}

	/// Check if `fields` contains any unknown fields (not native to Squarkdown).
	pub fn check_no_unknown<'a>(fields: impl Iterator<Item = &'a String>) -> SquarkResult
	{
		let mut errs = SquarkError::multiple("");

		for field in fields {
			if [
				"dest", "destination",
				"title",
				"desc", "description",
				"head", "heading",
				"capt", "caption",
				"tags",
				"date",   "release-date",
				"update", "last-update",
				"clean",  "cleanse",
			].contains(&field.as_ref()) {
				continue;
			}

			errs.push(SquarkError::Recoverable {
				msg: fmt!("warning: unknown field {W}{field}"),
				hint: fmt!("only Squarkdown-native fields like {W}desc{G} or {W}update{G} are allowed before {W}---"),
				debug: vec![],
			});
		}

		errs.or(())
	}

	#[must_use]
	pub fn shard_for(filepath: &Path, config: &SquarkupConfig) -> String
	{
		utils::display_rel(filepath, &config.paths.root)
	}

	/// Extract a multi-valued field from `fields`.
	fn take(fields: &mut HashMap<String, Strings>, long: &str, short: &str) -> Option<Vec<String>>
	{
		fields.remove(short)
			.or_else(|| fields.remove(long))
			.map(|t| t.into_vec())
	}

	/// Extract a single-valued field from `fields` by joining the values with ` / `.
	fn take_flat(fields: &mut HashMap<String, Strings>, long: &str, short: &str) -> Option<String>
	{
		Some(
			fields.remove(short)
				.or_else(|| fields.remove(long))?
				.join(" / ")
		)
	}

	fn try_parse_date(date: &str, field: &str) -> SquarkResult<Date>
	{
		let date = date.replace("winter", "December");
		let date = date.replace("spring", "March");
		let date = date.replace("summer", "June");
		let date = date.replace("fall",   "September");
		let date = date.replace("autumn", "September");

		let long  = format_description!("[year] [month repr:long] [day padding:none]");
		let short = format_description!("[year] [month repr:short] [day padding:none]");

		Err(())
			.or_else(|_| Date::parse(&date, long))
			.or_else(|_| Date::parse(&date, short))
			.or_else(|_| Date::parse(&fmt!("{date} 1"), long))
			.or_else(|_| Date::parse(&fmt!("{date} 1"), short))
			.or_else(|_| Date::parse(&fmt!("{date} January 1"), long))
			.map_err(|_| SquarkError::Recoverable {
				msg: fmt!("invalid date for {W}{field}{R}: {date}"),
				hint: str!("dates use the format {W}<year> <month?> <date?>"),
				debug: vec![]
			})
	}
}

/// Serialisation
impl PageData
{
	/// Serialise to serde applying `config`.
	pub fn serialize_with_config<S>(&self, s: S, config: &SquarkupConfig) -> Result<S::Ok, S::Error>
		where S: serde::Serializer
	{
		let mut map = s.serialize_map(None)?;

		macro_rules! ser {
			($field:ident => $val:expr) => {
				map.serialize_entry(self.$field(config.out.shorter_fields), $val)
			}
		}
	
		fn format_date(date: Date) -> String {
			date
				.format(format_description!("[year]-[month]-[day]"))
				.expect("date serialisation always succeeds")
		}
		
		ser!(path => &utils::display_rel(&self.filepath, &config.paths.root))?;
		ser!(dest => &utils::display_rel(&self.destination, &config.out.folder))?;

		ser!(flags => &self.flags)?;

		if let Some(v) = &self.title       { ser!(title => v)? }
		if let Some(v) = &self.description { ser!(desc => v)? }
		if let Some(v) = &self.heading     { ser!(head => v)? }
		if let Some(v) = &self.caption     { ser!(capt => v)? }

		ser!(tags => &self.tags)?;
		
		if let Some(v) =  self.release_date     { ser!(date => &format_date(v))? }
		if let Some(v) = &self.release_date_raw { ser!(date_raw => v)? }
		if let Some(v) =  self.last_update      { ser!(update => &format_date(v))? }
		if let Some(v) = &self.last_update_raw  { ser!(update_raw => v)? }

		for (key, val) in &self.other {
			map.serialize_entry(key, val)?;
		}

		map.end()
	}

	#[must_use]
	pub fn to_serializable<'d>(&'d self, config: &'d SquarkupConfig) -> SerializablePageData<'d>
	{
		SerializablePageData { page_data: self, config }
	}
}

macro_rules! impl_field_repr
{
	($field:ident => $both:literal) => {
		impl_field_repr!($field => $both, $both);
	};
	($field:ident => $short:literal, $full:literal) =>
	{
		pub fn $field(&self, short: bool) -> &str {
			if short {$short} else {$full}
		}
	};
}

impl PageData
{
	impl_field_repr!(path       => "path",       "filepath"        );
	impl_field_repr!(dest       => "dest",       "destination"     );
	impl_field_repr!(flags      => "flags"                         );
	impl_field_repr!(title      => "title"                         );
	impl_field_repr!(desc       => "desc",       "description"     );
	impl_field_repr!(head       => "head",       "heading"         );
	impl_field_repr!(capt       => "capt",       "caption"         );
	impl_field_repr!(tags       => "tags"                          );
	impl_field_repr!(date       => "date",       "release_date"    );
	impl_field_repr!(date_raw   => "date_raw",   "release_date_raw");
	impl_field_repr!(update     => "update",     "last_update"     );
	impl_field_repr!(update_raw => "update_raw", "last_update_raw" );
	impl_field_repr!(other      => "other"                         );
}


pub struct SerializablePageData<'d>
{
	page_data: &'d PageData,
	config:    &'d SquarkupConfig,
}

impl serde::Serialize for SerializablePageData<'_>
{
	fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
		where S: serde::Serializer
	{
		self.page_data.serialize_with_config(s, self.config)
	}
}
