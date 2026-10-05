use crate::config::*;
use crate::types::*;
use crate::utils;
use crate::colours::*;
use crate::macros::*;

use path_clean::PathClean;
use path_macro::path;
use time::Date;
use time::macros::format_description;

use std::collections::{ HashMap };
use std::path::{ Path, PathBuf };


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

		let mut destination = PathBuf::new();

		if let Some(dest) = Self::take_flat(&mut fields, "destination", "dest") {
			destination = path!(config.out.folder / utils::to_rel(&dest.replace(" ", "-"))).clean();

			if config.errors.strict && !destination.starts_with(&config.paths.root) {
				errs.push(SquarkError::Unrecoverable {
					msg: slash!("cannot export a file to: {}", destination),
					hint: fmt!("a file's destination directory must remain under the root directory of your project"),
					debug: vec![
						slash!("your project's root directory is: {}", config.paths.root),
						slash!("in file: {}", filepath),
					],
				});
			}
		}
		else {
			errs.push(SquarkError::Unrecoverable {
				msg: fmt!("missing field: {W}dest"),
				hint: fmt!("active pages must specify where they should be rendered to"),
				debug: vec![
					fmt!("parsed fields: {GREY1}{fields:?}")
				],
			});
		}
		
		let heading      = Self::take_flat(&mut fields, "heading", "head");
		let title        = Self::take_flat(&mut fields, "title", "title").or_else(|| heading.clone());
		
		let caption      = Self::take_flat(&mut fields, "caption", "capt");
		let description  = Self::take_flat(&mut fields, "description", "desc").or_else(|| caption.clone());

		let tags         = Self::take(&mut fields, "tags", "tags").unwrap_or_default();

		let mut release_date = None;
		let mut release_date_raw = None;

		if let Some(raw) = Self::take_flat(&mut fields, "release-date", "date") {
			catch!(errs => {
				release_date = Some(Self::try_parse_date(&raw, "release date")?);
			});
			release_date_raw = Some(raw);
		}

		let mut last_update = release_date;
		let mut last_update_raw = release_date_raw.clone();

		if let Some(raw) = Self::take_flat(&mut fields, "last-update", "update") {
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

impl PageData
{
	/// Serialise this page data to JSON with long, unabbreviated field names.
	#[must_use]
	pub fn serialised_long<'s>(&'s self, config: &SquarkupConfig) -> SerialisedPageData<'s>
	{
		SerialisedPageData {
			filepath:         utils::display_rel(&self.filepath, &config.paths.root),
			destination:      utils::display_rel(&self.destination, &config.out.folder),
			flags:            &self.flags,
			title:            self.title.as_ref(),
			description:      self.description.as_ref(),
			heading:          self.heading.as_ref(),
			caption:          self.caption.as_ref(),
			tags:             &self.tags,
			release_date:     self.release_date,
			release_date_raw: self.release_date_raw.as_ref(),
			last_update:      self.last_update,
			last_update_raw:  self.last_update_raw.as_ref(),
			other:            &self.other,
		}
	}
}


#[serde_with::skip_serializing_none]
#[derive(serde::Serialize)]
pub struct SerialisedPageData<'s>
{
	pub filepath: String,
	pub destination: String,
	pub flags: &'s [String],

	pub title:       Option<&'s String>,
	pub description: Option<&'s String>,
	pub heading:     Option<&'s String>,
	pub caption:     Option<&'s String>,
	
	pub tags: &'s [String],
	
	#[serde(serialize_with = "serialise_date")]
	pub release_date: Option<Date>,
	
	pub release_date_raw: Option<&'s String>,
	
	#[serde(serialize_with = "serialise_date")]
	pub last_update: Option<Date>,
	
	pub last_update_raw: Option<&'s String>,

	pub other: &'s HashMap<String, Strings>,
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

impl SerialisedPageData<'_>
{
	impl_field_repr!(filepath   => "path",       "filepath"        );
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


fn serialise_date<S>(date: &Option<Date>, serialiser: S) -> Result<S::Ok, S::Error>
	where S: serde::Serializer
{
	let Some(date) = date else {
		return serialiser.serialize_none();
	};

	let format = format_description!("[year]-[month]-[day]");
	let formatted = date.format(format).expect("date serialisation always succeeds");

	serialiser.serialize_str(&formatted)
}
