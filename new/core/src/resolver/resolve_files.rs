use path_slash::PathExt;

use crate::{
	SquarkupConfig, SquarkResult,
	macros::*,
};

use std::path::PathBuf;


/// Find all candidate files for squarkup in the user's project repo, as specified by their `config.paths.sources`, `.include_patterns` and `.exclude_patterns`.
pub fn resolve_files(config: &SquarkupConfig) -> impl Iterator<Item = SquarkResult<PathBuf>>
{
	// TODO root-only

	// if only we had `yield` generators syntax...
	config.paths.sources.iter().flat_map(|source|
		walkdir::WalkDir::new(&config.paths.root.join(source))
			.into_iter()

			// skip ignored folders and files
			.filter_entry(|e| should_include_path(e, config))

			.map(|e| dbg!(e))

			// yield `SquarkError::External` errors, not walkdir errors
			.map(|e| e.map_err(err!()))

			// don't yield folders, only yield files
			.filter(|e| !e.as_ref().is_ok_and(|entry| entry.path().is_dir()))

			// yield paths, not walkdir entries
			.map(|e| e.map(|entry| entry.path().to_path_buf()))
	)
}

/// Should `entry` be squarked up (file) or searched (folder), according to the user's squarkup `config`?
fn should_include_path(entry: &walkdir::DirEntry, config: &SquarkupConfig) -> bool
{
	// TODO symlinks?

	let path = entry.path();
	let path_str = path.to_slash().expect("path should not contain non-Unicode characters");

	if !config.paths.exclude_patterns.is_empty() {
		for pattern in &config.paths.exclude_patterns {
			if pattern.is_match(&path_str) {
				return false;
			}
		}
	}

	if path.is_file() && !config.paths.include_patterns.is_empty() {
		for pattern in &config.paths.include_patterns {
			if pattern.is_match(&path_str) {
				return true;
			}
		}

		return false;
	}

	true
}
