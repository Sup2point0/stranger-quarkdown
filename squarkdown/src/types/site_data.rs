use crate::prelude::*;
use crate::types::{
	Shard,
	page_data::SerializablePageData,
	series::Series,
};
use crate::utils;
use crate::utils::colours::*;
use crate::utils::macros::*;

use serde::ser::SerializeMap;
use time::{ UtcDateTime, macros::* };

use std::collections::{ HashSet, HashMap, BTreeMap };
use std::path::{ Path };
use std::sync::atomic::{ AtomicUsize };


/// Aggregate site data produced during squarkup.
/// 
/// This primarily consists of a lot of maps for lookup, for instance when resolving links.
#[derive(Debug, Default)]
pub struct SiteData
{
	pub stats: SiteStats,

	/// Data of all active pages in the site.
	pages: BTreeMap<Shard, PageData>,

	/// Series found during squarkup and the pages in each series.
	series: BTreeMap<String, Series>,

	/// Tags found during squarkup and the pages with each tag.
	tags: BTreeMap<String, HashSet<Shard>>,
}

#[derive(Debug, serde::Serialize)]
pub struct SiteStats
{
	#[serde(serialize_with = "serialise_datetime")]
	pub built_on: UtcDateTime,

	pub checked_files: usize,
	pub active_pages: usize,
	pub assets: usize,
	pub links: AtomicUsize,
	pub dead_links: AtomicUsize,
}

impl Default for SiteStats
{
	fn default() -> Self {
		Self {
			built_on: UtcDateTime::now(),
			checked_files: 0,
			active_pages: 0,
			assets: 0,
			links: AtomicUsize::new(0),
			dead_links: AtomicUsize::new(0),
		}
	}
}

impl SiteData
{
	pub fn new() -> Self {
		Self::default()
	}

	#[must_use]
	pub fn pages_map(&self) -> &BTreeMap<Shard, PageData> {
		&self.pages
	}

	pub fn pages(&self) -> impl Iterator<Item = &PageData> {
		self.pages.values()
	}

	#[must_use]
	pub fn get_page(&self, shard: &Shard) -> Option<&PageData>
	{
		self.pages.get(shard)
	}

	/// Set `page` as the index page for its series(s).
	/// 
	/// Errors if a conflict(s) is found where multiple pages want to be the index page for a particular series.
	pub fn add_index(&mut self, page: &PageData, config: &SquarkupConfig) -> SquarkResult
	{
		let mut errs = SquarkError::multiple("");

		for series in if !page.index.is_empty() {&page.index} else {&page.series}
		{
			let index = &mut self.series
				.entry(series.clone()).or_default()
				.index_page;

			/* Check for conflicts where multiple pages want to be the index page for a tag */
			if !index.is_empty()
			{
				let col = if config.errors.strict {R} else {W};

				let msg = fmt!(
					"Found conflicting index pages: {W}{index}{col} and {W}{}{col} both want to be the index page for the {W}{BOLD}{series}{UNBOLD}{col} series",
					page.shard,
				);
				let hint = str!("Each series can only have 1 index page");
				let debug = vec![];

				errs.push(
					if config.errors.strict {
						SquarkError::Recoverable { msg, hint, debug }
					} else {
						SquarkError::Warning { msg, hint, debug }
					}
				);
			}

			*index = page.shard.clone();
		}

		errs.or(())
	}

	/// Add a page's metadata to the site.
	pub fn add_page(&mut self, page: PageData)
	{
		let shard = &page.shard;

		for tag in &page.tags {
			self.tags.entry(tag.clone()).or_default().insert(shard.clone());
		}
		for series in &page.series {
			self.series.entry(series.clone()).or_default().pages.insert(shard.clone());
		}

		self.pages.insert(shard.clone(), page);
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
				recoverable! {
					msg: fmt!(
						"Found conflicting pages: {W}{}{R} and {W}{}{R} both want to export to {W}{}",
						utils::display_rel(&existing.filepath, &config.paths.root),
						utils::display_rel(&page.filepath, &config.paths.root),
						utils::display_rel(dest, &config.paths.site),
					),
					hint: str!("Pages must have unique export destinations, otherwise one will overwrite the other"),
					debug: vec![],
				}
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
		let pages: BTreeMap<&Shard, SerializablePageData>
			= self.pages.iter()
				.map(|(shard, page)| (shard, page.to_serializable(config)))
				.collect();

		let mut out = s.serialize_map(None)?;
		out.serialize_entry("stats", &self.stats)?;
		out.serialize_entry("pages", &pages)?;
		out.serialize_entry("series", &self.series)?;
		out.serialize_entry("tags", &self.tags)?;
		out.end()
	}

	#[must_use]
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
