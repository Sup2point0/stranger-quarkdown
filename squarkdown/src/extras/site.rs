use crate::prelude::*;
use crate::utils::log;
use crate::utils::colours::*;
use crate::utils::macros::*;

use regex::regex;

use std::fs::{ self, File };
use std::io::BufWriter;
use std::path::{ Path };


pub fn save_site_data(dest: &Path, site_data: &SiteData, config: &SquarkupConfig) -> SquarkResult
{
	if config.errors.strict && !dest.starts_with(&config.paths.root) {
		SquarkError::Recoverable {
			msg: slash!("cannot export site data to: {}", dest),
			hint: fmt!("output files must remain under your project root when {Y}errors.strict{G} is enabled"),
			debug: vec![],
		}.depends(&config)?;
	}

	let data_raw = site_data.to_serializable(&config);
	let file = BufWriter::new(File::create(dest)?);
	serde_json::to_writer_pretty(file, &data_raw)?;

	// unindent arbitrary fields (which are always lists)
	let data = fs::read_to_string(dest)?;
	let data = regex!("\\[\n {8}\"").replace_all(&data, "[\"");
	let data = regex!(",\n {8}\"").replace_all(&data, ", \"");
	let data = regex!("\"\n {6}\\]").replace_all(&data, "\"]");
	fs::write(&dest, data.as_bytes())?;

	log::ok!(slash!("saved site data to: {B}{}", dest));

	Ok(())
}
