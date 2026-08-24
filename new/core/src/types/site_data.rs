use time::{ UtcDateTime };

use super::PageData;

use std::collections::HashMap;
use std::path::PathBuf;


#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct SiteData
{
	/// The active pages in the site, keyed by the location (filepath) of their source file.
	/// 
	/// Since the filepath of any file must be unique, this reliably identifies files with minimal effort!
	pub pages: HashMap<PathBuf, PageData>,

	/// Maps tags to the pages that included them.
	pub tags: HashMap<String, Vec<PathBuf>>,

	pub stats: SiteStats,
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

	/// Add a page's metadata to the site.
	pub fn add_page(&mut self, filepath: PathBuf, page_data: PageData)
	{
		for tag in &page_data.tags {
			self.tags
				.entry(tag.clone()).or_default()
				.push(filepath.clone());
		}

		self.pages.insert(filepath, page_data);
	}
}
