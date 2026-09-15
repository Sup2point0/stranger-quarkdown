use time::{ UtcDateTime };

use super::PageData;

use std::collections::HashMap;
use std::fs;
use std::path::{ Path };


#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct SiteData
{
	/// The active pages in the site, keyed by the location (filepath) of their source file.
	/// 
	/// Since the filepath of any file must be unique, this reliably identifies files with minimal effort!
	pages: HashMap<String, PageData>,

	/// Maps tags to the pages that included them.
	tags: HashMap<String, Vec<String>>,

	stats: SiteStats,
}

#[derive(Clone, Debug, serde::Serialize)]
pub struct SiteStats
{
	built_on: UtcDateTime,
	source_files: usize,
	active_pages: usize,
}

impl Default for SiteStats
{
	fn default() -> Self {
		Self {
			built_on: UtcDateTime::now(),
			source_files: 0,
			active_pages: 0,
		}
	}
}

impl SiteData
{
	pub fn new() -> Self {
		Default::default()
	}

	pub fn pages(&self) -> impl Iterator<Item = &PageData>
	{
		self.pages.values()
	}

	pub fn get_page(&self, path: impl AsRef<Path>) -> Option<&PageData>
	{
		self.pages.get(&Self::normalise_path(path))
	}

	/// Add a page's metadata to the site.
	pub fn add_page(&mut self, page_data: PageData)
	{
		let key = Self::normalise_path(&page_data.filepath);

		for tag in &page_data.tags {
			self.tags
				.entry(tag.clone()).or_default()
				.push(key.clone());
		}

		self.pages.insert(key, page_data);
	}

	pub fn normalise_path(path: impl AsRef<Path>) -> String
	{
		let absolute = fs::canonicalize(path.as_ref())
			.expect("file data is always sourced from a real file");

		let slashed = path_slash::PathBufExt::to_slash(&absolute).unwrap();

		slashed.to_string()
	}
}
