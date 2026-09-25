use crate::prelude::*;

use path_macro::path;

use std::path::{ PathBuf };


/// Find all assets to copy to SvelteKit's `static/` directory, as specified by the user's `config.assets.folder`, `.site-assets-folder` and `.extensions`.
/// 
/// Returns an iterator of `(source, dest)` pairs.
pub fn resolve_assets(config: &SquarkupConfig) -> impl Iterator<Item = SquarkResult<(PathBuf, PathBuf)>>
{
	walkdir::WalkDir::new(&config.assets.folder)
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

				let path_rel = config.assets.rel_path(&path)
					.expect("walkdir entries are guaranteed to be under `assets-folder`");

				let dest = path!(config.paths.site / "static" / path_rel);

				Ok((path, dest))
			},
			Err(e) => Err(SquarkError::external(e)),
		})
}
