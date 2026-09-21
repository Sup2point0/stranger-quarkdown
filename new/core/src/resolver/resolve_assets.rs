use crate::core::*;

use std::path::PathBuf;


/// Find all assets to copy to SvelteKit's `static/` directory, as specified by the user's `config.assets.folder`, `.site-assets-folder` and `.extensions`.
pub fn resolve_raw_assets(config: &SquarkupConfig) -> impl Iterator<Item = SquarkResult<PathBuf>>
{
	config.assets.folder.iter().flat_map(|assets_folder|
	{
		walkdir::WalkDir::new(assets_folder)
			.into_iter()

			// don't yield folders, only yield files
			.filter(|e|
				!e.as_ref().is_ok_and(|entry| should_exclude(entry, config))
			)

			// yield paths, not walkdir entries
			.map(|entry| match entry {
				Ok(e) => Ok(e.into_path()),
				Err(e) => Err(SquarkError::external(e)),
			})
	})
}

fn should_exclude(entry: &walkdir::DirEntry, config: &SquarkupConfig) -> bool
{
	if !entry.file_type().is_file() {
		return true;
	}

	let path = entry.path();

	if let Some(site_assets_folder) = &config.assets.site_assets_folder
	&& path.starts_with(site_assets_folder) {
		return true;
	}

	// TODO check extensions

	false
}
