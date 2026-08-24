use super::ResolutionError;
use crate::SquarkupConfig;
use crate::utils::macros::*;

use std::path::PathBuf;


/// Find the root directory of the user's project to squarkup.
/// 
/// This is defined to be the closest directory that contains a `.squarkdown/` folder.
pub fn resolve_project_root() -> Result<PathBuf, ResolutionError>
{
	let Ok(cwd) = std::env::current_dir() else {
		return Err(ResolutionError::ReadError { target: str!("current working directory") });
	};

	for dir in cwd.ancestors() {
		if all!(dir.join(".squarkdown") => .exists(), .is_dir()) {
			return Ok(dir.to_owned());
		}
		if all!(dir.join("squarkup.toml") => .exists(), .is_file()) {
			return Ok(dir.to_owned());
		}
		if all!(dir.join("squarkup.json") => .exists(), .is_file()) {
			return Ok(dir.to_owned());
		}
	}

	Err(ResolutionError::NoSquarkdown)
}


pub fn find_files(config: &SquarkupConfig) -> Vec<PathBuf> { unimplemented!() }
