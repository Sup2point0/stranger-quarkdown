use crate::{
	SquarkResult, SquarkupConfig,
	utils::macros::*,
};

use std::path::PathBuf;


/// Find all candidate files for squarkup in the user's project repo, as specified by their `paths.sources`, `paths.include` and `paths.exclude`.
pub fn find_files(config: &SquarkupConfig) -> impl Iterator<Item = SquarkResult<PathBuf>>
{
	walkdir::WalkDir::new(&config.paths.root)
		.into_iter()
		.filter_entry(|e| include_dir(e, config))
		.map(|e| e.map_err(err!()))
		.map(|e| e.map(|entry| entry.path()))
}

fn include_dir(entry: &walkdir::DirEntry, config: &SquarkupConfig) -> bool
{
	// TODO
	true
}
