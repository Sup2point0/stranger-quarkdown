mod parser_core;
mod charm_parser; pub use charm_parser::{ CharmParser };
mod shared; use shared::{ ctx };
mod ctx; pub use ctx::{ ParseCtx };

#[cfg(test)] mod test_utils;


use crate::core::*;
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
	/* Read up until we see a `-->` terminating the charm squark */
	let mut reader = BufReader::new(File::open(&filepath)?);
	let mut chunk = str!();
	let mut source = str!();

	loop {
		chunk.clear();
		reader.read_line(&mut chunk)?;
		source.push_str(&chunk);

		if chunk.contains("-->") {
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
