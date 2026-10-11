use crate::prelude::*;
use crate::log;
use crate::colours::*;
use crate::macros::*;

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

	log::ok!(slash!("saved site data to: {B}{}", dest));

	Ok(())
}
