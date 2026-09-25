use crate::prelude::*;
use crate::macros::*;

use path_macro::path;

use std::path::{ Path, PathBuf };


/// Find all assets to copy to SvelteKit's `static/` directory, as specified by the user's `config.assets.folder`, `.site-assets-folder` and `.extensions`.
/// 
/// Returns an iterator of `(source, dest)` pairs.
pub fn resolve_assets(config: &SquarkupConfig) -> impl Iterator<Item = SquarkResult<(PathBuf, PathBuf)>>
{
	config.assets.folder.iter().flat_map(move |assets_folder|
	{
		walkdir::WalkDir::new(assets_folder)
			.into_iter()

		.filter(|entry| match entry {
			Err(..) => true,
			Ok(e) =>
				e.file_type().is_file()
				&& config.assets.has_asset_extension(e.path()),
		})

			.map(move |entry| match entry {
				Ok(e) => {
					let path = e.into_path();
					let dest = resolve_dest(&path, assets_folder, config)?;
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

	if let Some(extension) = path.extension() {
		return config.assets.extensions.iter().all(|ext| **ext != *extension);
	}

	true
}

fn resolve_dest(
	path: &Path,
	assets_folder: &Path,
	config: &SquarkupConfig,
) -> SquarkResult<PathBuf>
{
	let base = {
		if let Some(site_assets_folder) = &config.assets.site_assets_folder
		&& path.starts_with(site_assets_folder)
		{
			site_assets_folder
		} else {
			assets_folder
		}
	};

	let path_rel = path.strip_prefix(base)?;
	let dest = path!(config.paths.site / "static" / path_rel);

	Ok(dest)
}
