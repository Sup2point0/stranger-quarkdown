use super::PageData;
use crate::core::*;
use crate::utils;
use crate::colours::*;
use crate::macros::*;

use time::{ UtcDateTime };

use std::collections::HashMap;
use std::path::PathBuf;


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
	pub built_on: UtcDateTime,
	pub active_pages: usize,
}

impl Default for SiteStats
{
	fn default() -> Self {
		Self {
			built_on: UtcDateTime::now(),
			active_pages: 0,
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

	/// Check if there are active pages in the site that conflict (want to export to the same destination folder).
	pub fn check_conflicts(&self, config: &SquarkupConfig) -> SquarkResult
	{
		let mut seen_dests = HashMap::<PathBuf, &PageData>::new();

		for page in self.pages() {
			let Ok(dest) = dunce::canonicalize(&page.destination) else { continue };

			if let Some(conflict) = seen_dests.get(&dest) {
				return Err(SquarkError::Unrecoverable {
					msg: fmt!(
						"found conflicting pages: {W}{}{R} and {W}{}{R} both want to export to {W}{}",
						utils::display_rel(&conflict.filepath, &config.paths.root),
						utils::display_rel(&page.filepath, &config.paths.root),
						utils::display_rel(dest, &config.paths.site),
					),
					hint: str!(),
					debug: vec![],
				});
			}

			seen_dests.insert(dest, page);
		}

		Ok(())
	}
}

impl SiteData
{
	#[must_use]
	pub fn serialise(self, config: &SquarkupConfig) -> impl serde::Serialize
	{
		serde_json::json!({
			"pages":
				self.pages.into_iter()
				.map(|(key, page)|
					(key, page.serialise(config))
				)
				.collect::<HashMap<_, _>>(),

			"tags": self.tags,
			"stats": self.stats,
		})
	}
}
