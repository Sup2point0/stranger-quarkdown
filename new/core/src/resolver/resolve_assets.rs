use crate::prelude::*;
use crate::macros::*;

use path_macro::path;

use std::path::PathBuf;


/// Find all assets to copy to SvelteKit's `static/` directory, as specified by the user's `config.assets.folder`, `.site-assets-folder` and `.extensions`.
pub fn resolve_raw_assets(config: &SquarkupConfig) -> impl Iterator<Item = SquarkResult<(PathBuf, PathBuf)>>
{
	config.assets.folder.iter().flat_map(move |assets_folder|
	{
		walkdir::WalkDir::new(assets_folder)
			.into_iter()

			// don't yield folders, only yield files
			.filter(|e|
				!e.as_ref().is_ok_and(|entry| should_exclude(entry, config))
			)

			// yield paths, not walkdir entries
			.map(move |entry| match entry {
				Ok(e) => {
					let path = e.into_path();
					let path_rel = path.strip_prefix(assets_folder).map_err(err!())?;
					let dest = path!(config.paths.site / "static" / path_rel);

					Ok((path, dest))
				},
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
