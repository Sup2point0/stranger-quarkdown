use crate::prelude::*;
use crate::log;
use crate::macros::*;

use path_macro::path;
use path_slash::PathExt;

use std::path::PathBuf;


/// Find all candidate files for squarkup in the user's project repo, as specified by their `config.paths.sources`, `.include` and `.exclude`.
pub fn resolve_files(config: &SquarkupConfig) -> impl Iterator<Item = SquarkResult<PathBuf>>
{
	// if only we had `yield` generators syntax...
	config.paths.sources.iter().flat_map(|source|
	{
		/* NOTE: "/" is a special case that means 'root-only', without recursing into directories */
		let walker = {
			if *source == config.paths.root {
				log::info!(slash!("searching non-recursively from: {}", config.paths.root));
				walkdir::WalkDir::new(&config.paths.root).max_depth(1)
			} else {
				let path = path!(config.paths.root / source);
				log::info!(slash!("searching recursively from: {}", path));
				walkdir::WalkDir::new(path)
			}
		};

		walker
			.into_iter()

			// skip ignored folders and files
			.filter_entry(|e| should_include(e, config))

			// don't yield folders, only yield files
			.filter(|e| !e.as_ref().is_ok_and(|entry| entry.file_type().is_dir()))

			.map(|e| e
				.map(walkdir::DirEntry::into_path)
				.map_err(SquarkError::external)
			)
	})
}

/// Should `entry` be squarked up (file) or searched (folder), according to the user's squarkup `config`?
fn should_include(entry: &walkdir::DirEntry, config: &SquarkupConfig) -> bool
{
	let path = entry.path();
	let path_str = path.to_slash().expect("Squarkdown does not support non-Unicode filepaths");

	if config.paths.exclude.iter().any(|p| p.is_match(&path_str)) {
		return false;
	}

	if entry.file_type().is_file() && !config.paths.include.is_empty() {
		return config.paths.include.iter().any(|p| p.is_match(&path_str));
	}

	true
}
