use crate::{
	SquarkupConfig, SquarkResult, SquarkError,
	log,
	colours::*,
	macros::*,
};

use std::error::Error;
use std::fs::{ File };
use std::io::{ Read };
use std::path::{ PathBuf, Path };


enum Extension { TOML, JSON }


/// Find, read, load, validate and initialise the user's squarkup configuration, either from `squarkup.toml` or `squarkup.json`.
pub fn resolve_config(root: PathBuf) -> SquarkResult<SquarkupConfig>
{
	log::is!("resolving config...");

	let (filepath, ext) = find_config(&root)?;
	log::ok!(slash!("found your squarkup config: {BLUE}{}", filepath));

	let config = match ext {
		Extension::TOML => {
			log::info!("reading config...");
			let data = read_toml_config(filepath)?;
			log::info!("read successful!");
			log::info!("validating config...");
			SquarkupConfig::try_from_toml(data, &root)
		},
		Extension::JSON => {
			unimplemented!()
		},
	}?;

	log::ok!("config looks good, all set!");

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
		hint: format!("make sure you have either a {WHITE}.squarkdown/{GREEN} folder, or a {WHITE}squarkup.toml{GREEN} or {WHITE}squarkup.json{GREEN} file, in the root of your project"),
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
