use crate::{
	SquarkupConfig, SquarkResult, SquarkError,
	utils::colours::*,
	utils::macros::*,
};

use std::error::Error;
use std::fs::{ File };
use std::io::{ Read };
use std::path::{ PathBuf, Path };


enum Extension { TOML, JSON }


/// Find, read, load and validate the user's squarkup configuration, either in `squarkup.toml` or `squarkup.json`.
pub fn resolve_config(root: PathBuf) -> SquarkResult<SquarkupConfig>
{
	let (filepath, ext) = find_config(&root)?;

	let mut config = SquarkupConfig::init_defaults(root.clone());

	match ext {
		Extension::TOML => {
			let data = read_toml_config(filepath)?;
			config.set_from_toml(data)?;
		},
		Extension::JSON => {
			unimplemented!()
		},
	}

	Ok(config)
}

/// Find the location of the user's squarkup configuration, in `(.squarkdown/)?squarkup.(toml|json)`.
fn find_config(root: &Path) -> SquarkResult<(PathBuf, Extension)>
{
	if let Some(out) = find_config_from(&dir!(root / ".squarkdown/")) {
		return Ok(out);
	}

	if let Some(out) = find_config_from(root) {
		return Ok(out);
	};

	Err(SquarkError::Unrecoverable {
		msg: str!("could not find your squarkup configuration file"),
		hint: format!("make sure you have either a {WHITE}.squarkdown/{RED} folder, or a {WHITE}squarkup.toml{RED} or {WHITE}squarkup.json{RED} file, in the root of your project"),
		debug: vec![format!(
			"looked in {WHITE}{}{GREY} and {WHITE}{}",
			root.display(),
			dir!(root / ".squarkdown/").display(),
		)],
	})
}

fn find_config_from(folder: &Path) -> Option<(PathBuf, Extension)>
{
	let file = folder.join("squarkup.toml");
	if file.exists() && file.is_file() {
		return Some((file, Extension::TOML));
	}
	
	let file = folder.join("squarkup.json");
	if file.exists() && file.is_file() {
		return Some((file, Extension::JSON));
	}

	// TODO: funny messages on close matches
	// if glob::glob(folder.join(""))

	None
}

/// Read the user's squarkup configuration from `squarkup.toml`.
fn read_toml_config(filepath: PathBuf) -> SquarkResult<toml::Table>
{
	(|| -> Result<toml::Table, Box<dyn Error>>
	{
		let mut file = File::open(&filepath)?;

		let mut content = str!();
		file.read_to_string(&mut content)?;

		let data = content.parse::<toml::Table>()?;
		Ok(data)
	})
	().map_err(|e| SquarkError::External(e))
}
