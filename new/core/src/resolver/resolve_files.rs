use crate::{
	SquarkupConfig, SquarkResult,
	macros::*,
};

use std::path::PathBuf;


/// Find all candidate files for squarkup in the user's project repo, as specified by their `paths.sources`, `paths.include` and `paths.exclude`.
/// 
/// The entire function `Err`s if `config.include` or `.exclude` have invalid entries that fail to compile. Each individual iterator entry may `Err` if reading the directory fails.
pub fn find_files(config: &mut SquarkupConfig)
	-> SquarkResult<impl Iterator<Item = SquarkResult<PathBuf>>>
{
	// TODO root-only

	config.compile_patterns();

	Ok(
		config.paths.sources.iter()
		.flat_map(|source|
			walkdir::WalkDir::new(&config.paths.root.join(source))
				.into_iter()
				.filter_entry(|e| should_include_path(e, config))
				.map(|e| e.map_err(err!()))
				.map(|e| e.map(|entry| entry.path().to_path_buf()))
		)
	)
}

fn should_include_path(entry: &walkdir::DirEntry, config: &SquarkupConfig) -> bool
{
	// TODO symlinks?

	let path = entry.path();
	let path_str = path.to_str().unwrap();

	// TODO exclude

	if path.is_file() && !config.paths.include.is_empty() {
		for pattern in &config.paths.include_patterns {
			if pattern.is_match(&path_str) {
				return true;
			}
		}

		return false;
	}

	true
}
