use toml::Table;

use super::{ ResolutionResult, ResolutionError };
use crate::SquarkupConfig;

use std::fs::File;
use std::path::{ PathBuf, Path };


/// Find, load and validate the user's squarkup configuration, either in `squarkup.toml` or `squarkup.json`.
pub fn resolve_config(root: PathBuf) -> ResolutionResult<SquarkupConfig>
{
	let filepath = find_config(&root)?;
	unimplemented!()
}

/// Find the location of the user's squarkup configuration, in `(.squarkdown/)?squarkup.(toml|json)`.
fn find_config(root: &Path) -> ResolutionResult<PathBuf>
{
	let folder = root.join(".squarkdown");

	if folder.exists() && folder.is_dir() {
		let file = folder.join("squarkup.toml");
		if file.exists() && file.is_file() {
			return Ok(file);
		}
		
		let file = folder.join("squarkup.json");
		if file.exists() && file.is_file() {
			return Ok(file);
		}
	}
	
	let file = root.join("squarkup.toml");
	if file.exists() && file.is_file() {
		return Ok(file);
	}
	
	let file = root.join("squarkup.json");
	if file.exists() && file.is_file() {
		return Ok(file);
	}

	Err(ResolutionError::NoConfig)
}

fn load_config(filepath: PathBuf) -> ResolutionResult<SquarkupConfig>
{
	let file = File::open(filepath)?;
	let content = file.

	unimplemented!()
}
