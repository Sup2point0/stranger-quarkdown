use super::*;

use std::path::{ Path };


impl AssetsConfig
{
	/// Does `path` have a file extension allowed by `config.assets.extensions`?
	pub fn has_asset_extension(&self, path: &Path) -> bool
	{
		if self.extensions.is_empty() {
			true
		}
		else if let Some(extension) = path.extension() {
			self.extensions.iter().any(|ext| **ext == *extension)
		}
		else {
			false
		}
	}

	/// Return `path` relative to either `assets.folder` or `assets.site-assets-folder`.
	pub fn rel_path<'s>(&'s self, path: &'s Path) -> Option<&'s Path>
	{
		let base = {
			if let Some(s) = &self.site_assets_folder
			&& path.starts_with(s)
			{
				s
			} else {
				&self.folder
			}
		};

		path.strip_prefix(base).ok()
	}
}
