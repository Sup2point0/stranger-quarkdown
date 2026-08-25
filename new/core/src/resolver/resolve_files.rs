use crate::{
	types::{ SquarkupConfig, ErrorAction },
	errors::{ SquarkResult, SquarkError },
	utils::log,
	utils::macros::*,
};

use std::path::PathBuf;


/// Find all candidate files for squarkup in the user's project repo, as specified by their `paths.sources`, `paths.include` and `paths.exclude`.
/// 
/// The entire function `Err`s if `config.source`, `.include` or `.exclude` have invalid entries. Each individual iterator entry may `Err` if reading fails.
pub fn find_files(config: &mut SquarkupConfig)
	-> SquarkResult<impl Iterator<Item = SquarkResult<PathBuf>>>
{
	// TODO root-only

	if !config.paths.include.is_empty() {
		let mut errs = vec![];

		for pattern in &config.paths.include {
			match regex::Regex::new(&pattern) {
				Ok(compiled) => config.paths.include_patterns.push(compiled),
				Err(e) => errs.push(SquarkError::external(e)),
			}
		}

		if !errs.is_empty() {
			match config.errors.on_error {
				ErrorAction::WARN => {
					for err in errs {
						log::bad!(err)
					}
				},
				ErrorAction::KILL => return Err(SquarkError::ManyRecoverable { errs }),
			}
		}
	}

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
