//! This module implements the parser for the charm squark, which turns `<!-- #SQUARK -->` metadata into a [`PageData`] object.

mod parser_core;

mod charm_parser;
pub use charm_parser::{ CharmParser };

mod ctx;
pub(crate) use ctx::{ ParseCtx };
pub(super) use ctx::{ ctx };

#[cfg(test)]
mod test_utils;


// == PUBLIC == //

use crate::prelude::*;
use crate::macros::*;

use std::fs::File;
use std::io::{ BufReader, BufRead };
use std::path::Path;


/// Parse the charm squark of the file at `filepath`, returning `Some(PageData)` for an active page, and `None` otherwise.
pub fn parse(
	filepath: impl AsRef<Path>,
	config: &SquarkupConfig,
) -> SquarkResult<Option<PageData>>
{
	let mut reader = BufReader::new(File::open(&filepath)?);
	let mut source = str!();
	
	/* Read up until we see a `-->` terminating the charm squark */
	loop {
		let i = source.len();

		if reader.read_line(&mut source)? == 0 {
			break;
		}

		if source[i..].contains("-->") {
			break;
		}
	}

	let parser = CharmParser::new(&source, filepath.as_ref().to_path_buf(), config);
	
	match parser.parse()
	{
		Ok(page) => Ok(Some(page)),
		Err(SquarkError::ABANDON) => Ok(None),
		Err(e) => Err(e)
	}
}
