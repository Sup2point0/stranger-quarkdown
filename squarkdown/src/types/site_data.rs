use crate::prelude::*;
use crate::types::page_data::SerializablePageData;
use crate::utils;
use crate::colours::*;
use crate::macros::*;

use serde::ser::SerializeMap;
use time::{ UtcDateTime, macros::* };

use std::collections::{ HashMap };
use std::path::{ Path };


#[derive(Clone, Debug, Default)]
pub struct SiteData
{
	pub stats: SiteStats,

	/// Maps tags to the pages that included them.
	tags: HashMap<String, Vec<String>>,

	/// The active pages in the site, keyed by the location (filepath) of their source file.
	/// 
	/// Since the filepath of any file must be unique, this reliably identifies files with minimal effort!
	pages: HashMap<String, PageData>,
}

#[derive(Clone, Debug, serde::Serialize)]
pub struct SiteStats
{
	#[serde(serialize_with = "serialise_datetime")]
	pub built_on: UtcDateTime,

	pub checked_files: usize,
	pub active_pages: usize,
	pub assets: usize,
}

impl Default for SiteStats
{
	fn default() -> Self {
		Self {
			built_on: UtcDateTime::now(),
			checked_files: 0,
			active_pages: 0,
			assets: 0,
		}
	}
}

impl SiteData
{
	pub fn new() -> Self {
		Self::default()
	}

	pub fn pages(&self) -> impl Iterator<Item = &PageData> {
		self.pages.values()
	}

	#[must_use]
	pub fn get_page(&self, key: &str) -> Option<&PageData>
	{
		self.pages.get(key)
	}

	/// Add a page's metadata to the site.
	pub fn add_page(&mut self, page_data: PageData)
	{
		let key = &page_data.shard;

		for tag in &page_data.tags {
			self.tags
				.entry(tag.to_owned()).or_default()
				.push(key.to_owned());
		}

		self.pages.insert(key.to_owned(), page_data);
		self.stats.active_pages += 1;
	}

	/// Check if there are conflicting active pages in the site that export to the same destination folder (which would mean one overwrites the other).
	pub fn check_conflicts(&self, config: &SquarkupConfig) -> SquarkResult
	{
		let mut seen_dests = HashMap::<&Path, &PageData>::new();

		for page in self.pages() {
			let dest = &page.destination;

			if let Some(existing) = seen_dests.insert(dest, page)
			{
				return Err(SquarkError::Recoverable {
					msg: fmt!(
						"found conflicting pages: {W}{}{R} and {W}{}{R} both want to export to {W}{}",
						utils::display_rel(&existing.filepath, &config.paths.root),
						utils::display_rel(&page.filepath, &config.paths.root),
						utils::display_rel(dest, &config.paths.site),
					),
					hint: str!("pages must have unique export destinations, otherwise one will overwrite the other"),
					debug: vec![],
				});
			}
		}

		Ok(())
	}
}

impl SiteData
{
	/// Serialise to serde applying `config`.
	pub fn serialize_with_config<S>(&self, s: S, config: &SquarkupConfig) -> Result<S::Ok, S::Error>
		where S: serde::Serializer
	{
		let pages: HashMap<&String, SerializablePageData>
			= self.pages.iter()
				.map(|(shard, page)| (shard, page.to_serializable(config)))
				.collect();

		let mut out = s.serialize_map(None)?;
		out.serialize_entry("stats", &self.stats)?;
		out.serialize_entry("pages", &pages)?;
		out.serialize_entry("tags", &self.tags)?;
		out.end()
	}

	pub fn to_serializable<'d>(&'d self, config: &'d SquarkupConfig) -> SerializableSiteData<'d>
	{
		SerializableSiteData { site_data: self, config }
	}
}

fn serialise_datetime<S>(date: &UtcDateTime, serialiser: S) -> Result<S::Ok, S::Error>
	where S: serde::Serializer
{
	let format = format_description!("[year]-[month]-[day] [hour]:[minute]:[second]");
	let formatted = date.format(format).expect("date serialisation always succeeds");

	serialiser.serialize_str(&formatted)
}


pub struct SerializableSiteData<'d> {
	site_data: &'d SiteData,
	config:    &'d SquarkupConfig,
}

impl serde::Serialize for SerializableSiteData<'_> {
	fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
		where S: serde::Serializer
	{
		self.site_data.serialize_with_config(s, self.config)
	}
}
