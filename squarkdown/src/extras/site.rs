use crate::prelude::*;
use crate::utils::log;
use crate::utils::colours::*;
use crate::utils::macros::*;

use regex::regex;

use std::fs::{ self, File };
use std::io::Read;
use std::io::{ BufWriter, Cursor };
use std::path::{ Path };


pub fn save_site_data(
	dest: &Path,
	site_data: &SiteData,
	config: &SquarkupConfig,
	dry_run: bool,
) -> SquarkResult
{
	if config.errors.strict && !dest.starts_with(&config.paths.root) {
		SquarkError::Recoverable {
			msg: slash!("cannot export site data to: {}", dest),
			hint: fmt!("output files must remain under your project root when {Y}errors.strict{G} is enabled"),
			debug: vec![],
		}.depends(config)?;
	}

	let data_raw = site_data.to_serializable(config);

	let mut data = str!();

	if dry_run {
		let mut cursor = Cursor::new(vec![]);
		serde_json::to_writer_pretty(&mut cursor, &data_raw)?;
		cursor.read_to_string(&mut data)?;
	}
	else {
		let file = BufWriter::new(File::create(dest)?);
		serde_json::to_writer_pretty(file, &data_raw)?;
		data = fs::read_to_string(dest)?;
	}

	// unindent arbitrary fields (which are always lists)
	let data = regex!("\\[\n {8}\"").replace_all(&data, "[\"");
	let data = regex!(",\n {8}\"").replace_all(&data, ", \"");
	let data = regex!("\"\n {6}\\]").replace_all(&data, "\"]");

	if dry_run {
		log::ok!(slash!("would save site data to: {B}{}", dest));
	} else {
		fs::write(dest, data.as_bytes())?;
		log::ok!(slash!("saved site data to: {B}{}", dest));
	}
	
	Ok(())
}
