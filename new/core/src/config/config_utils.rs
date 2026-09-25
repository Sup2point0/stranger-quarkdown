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

}
