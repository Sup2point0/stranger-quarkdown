use super::{ ResolutionResult, ResolutionError };
use crate::{
	SquarkupConfig, SquarkResult, SquarkError,
	utils::colours::*,
	utils::macros::*,
};

use std::fs::{ File };
use std::io::{ Read };
use std::path::{ PathBuf, Path };


/// Find, read, load and validate the user's squarkup configuration, either in `squarkup.toml` or `squarkup.json`.
pub fn resolve_config(root: PathBuf) -> SquarkResult<SquarkupConfig>
{
	let filepath = find_config(&root)?;
	// let defaults = load_config_defaults()?;
	let config = load_config(root, filepath);
	unimplemented!()
}

/// Find the location of the user's squarkup configuration, in `(.squarkdown/)?squarkup.(toml|json)`.
fn find_config(root: &Path) -> SquarkResult<PathBuf>
{
	if let Some(out) = find_config_from(&dir!(root / ".squarkdown/")) {
		return Ok(out);
	}

	if let Some(out) = find_config_from(root) {
		return Ok(out);
	};

	Err(SquarkError::Unrecoverable {
		msg: format!("could not find your {WHITE}squarkup.toml{RED} or {WHITE}squarkup.json{RED}"),
		debug: vec![format!(
			"looked in {WHITE}{}{GREY} and {WHITE}{}",
			root.display(),
			dir!(root / ".squarkdown/").display(),
		)],
	})
}

fn find_config_from(folder: &Path) -> Option<PathBuf>
{
	let file = folder.join("squarkup.toml");
	if file.exists() && file.is_file() {
		return Some(file);
	}
	
	let file = folder.join("squarkup.json");
	if file.exists() && file.is_file() {
		return Some(file);
	}

	None
}

/// Read and parse the user's squarkup configuration.
fn load_config(root: PathBuf, filepath: PathBuf) -> SquarkResult<SquarkupConfig>
{
	(|| -> Result<(), _> {
		let mut file = File::open(&filepath)?;

		let mut content = String::new();
		file.read_to_string(&mut content)?;

		let data = content.parse::<toml::Table>()?;

		Ok(())
	})().map_err(err!())?;

	SquarkupConfig::try_from_toml(data, root)
}
