//! This module implements the parser for the charm squark, which turns `<!-- #SQUARK -->` metadata into a [`PageData`] object.

mod parser_core;

mod charm_parser;
use charm_parser::{ CharmParser };

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


/// The maximum number of lines Squarkdown will check for a `<!--`.
const MAX_LINES_CHECKED: usize = 10;


/// Parse the charm squark of the file at `filepath`, returning `Some(PageData)` for an active page, and `None` otherwise.
pub fn parse(filepath: &Path, config: &SquarkupConfig) -> SquarkResult<Option<PageData>>
{
	let mut reader = BufReader::new(File::open(filepath)?);
	let mut source = str!();
	let mut may_have_charm_squark = false;
	let mut num_lines_checked = 0;
	
	/* Read up until we see a `-->` terminating the charm squark */
	loop {
		let i = source.len();

		if reader.read_line(&mut source)? == 0 {
			return Ok(None);
		}

		if !may_have_charm_squark && source[i..].contains("<--") {
			may_have_charm_squark = true;
		}
		if may_have_charm_squark && source[i..].contains("-->") {
			break;
		}
		
		/* If we still haven't seen `<!--` after reading `MAX_LINES_CHECKED` lines, bail out to avoid reading huge files into memory */
		num_lines_checked += 1;

		if !may_have_charm_squark && num_lines_checked > MAX_LINES_CHECKED {
			return Ok(None);
		}
	}

	let parser = CharmParser::new(&source, filepath.to_owned(), config);
	
	match parser.parse()
	{
		Ok(page) => Ok(Some(page)),
		Err(SquarkError::ABANDON) => Ok(None),
		Err(e) => Err(e)
	}
}
