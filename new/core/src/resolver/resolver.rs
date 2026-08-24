use super::ResolutionError;
use crate::SquarkupConfig;

use std::path::PathBuf;


/// Find the root directory of the user's project to squarkup.
/// 
/// This is defined to be the closest directory that contains a `.squarkdown/` folder.
pub fn resolve_project_root() -> Result<PathBuf, ResolutionError>
{
	let cwd = std::env::current_dir().or(Err(ResolutionError::NoSquarkdown))?;

	for dir in cwd.ancestors() {
		if dir.join(".squarkdown").exists() {
			return Ok(dir.to_owned());
		}
	}

	Err(ResolutionError::NoSquarkdown)
}


pub fn load_config(root: PathBuf) -> SquarkupConfig { unimplemented!() }
pub fn find_files(config: &SquarkupConfig) -> Vec<PathBuf> { unimplemented!() }
