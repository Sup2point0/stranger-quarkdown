use std::{
	fs::File,
	io::{ BufReader, BufRead, Read },
	iter,
	str::Chars,
};

use super::{ BufferedParser, ParseResult, ParseError };
use crate::{ log, SquarkupConfig, FileData };


/// A parser for the 'squark charm' header of a file.
pub struct CharmParser<'l, Source: Read = File>
{
	config: &'l SquarkupConfig,
	
	/// Have we encountered a `<!-- #SQUARK live!` yet?
	///
	/// If so, this means the user intends for the file to be squarked up, and error checking should be stricter to catch mistakes on their end.
	is_live: bool,

	// == INTERNALS == //
	
	/// The backing buffer that reads from the target file.
	_reader: BufReader<Source>,

	/// The current index in the current chunk the parser is pointing to.
	_index: usize,

	/* NOTE:
		Storing a `Chars` iterator over `_chunk_buffer` caused lifetime issues =(
		Having another `Vec<char>` is a little duplication, but it does make it much nicer to work with
	*/

	/// Individual characters of the currently in-memory chunk to process.
	_chunk: Vec<char>,
	
	/// The currently in-memory chunk to process.
	/// 
	/// This backs `._chunk`. Reuse this between line reads to avoid excessive allocations!
	_chunk_buffer: String,
}

/// The public parser interface.
impl<'l, Source: Read> CharmParser<'l, Source>
{
	pub fn init(file: Source, config: &'l SquarkupConfig) -> Self
	{
		let mut out = Self {
			config,
			_reader: BufReader::new(file),
			_index: 0,
			_chunk: vec![],
			_chunk_buffer: String::new(),
			is_live: false,
		};
		
		out.next_line();
		
		out
	}
	
	/// Run the parser to completion, extracting the metadata from the squark charm (if present) of the target file.
	pub fn parse(&mut self) -> Option<FileData>
	{
		match self._parse() {
			Ok(r) => Some(r),
			Err(ParseError::NoMatch) => None,
			Err(e) => {
				log::err(e);
				None
			},
		}
	}
}

/// *Specialised* parser internals, specialised to Squarkdown-Flavoured Markdown.
impl<'l, Source: Read> CharmParser<'l, Source>
{
	fn _parse(&mut self) -> Result<FileData, ParseError>
	{
		self.eat_spaces()?;
		
		let mut heading = None;
		
		if self.current() == Some('#') {
			heading = Some(self.parse_heading()?);
		}
		
		self.parse_charm()?;
		
		Ok(FileData {
			dest: "TODO".to_string(),
			heading,
		})
	}
	
	fn parse_heading(&mut self) -> ParseResult<String>
	{
		self.eat("#");
		unimplemented!()
	}
	
	fn parse_charm(&mut self) -> ParseResult
	{
		unimplemented!()
	}

	fn err(&self) -> ParseError
	{
		if self.is_live {
			ParseError::FatalEnd
		} else {
			ParseError::NoMatch
		}
	}
}

/// *Generic* parser internals, not specific to Squarkdown-Flavoured Markdown.
impl<'l, Source: Read> CharmParser<'l, Source>
{
	/// The current character in the source the parser is pointing to, or `None` if it is out of bounds.
	fn current(&self) -> Option<char>
	{
		self._chunk.get(self._index).map(|c| *c)
	}

	/// Read the next line of the source text into memory.
	fn next_line(&mut self) -> ParseResult
	{
		match self._reader.read_line(&mut self._chunk_buffer) {
			Ok(0) => return Err(ParseError::EndOfFile),
			Err(_) => return Err(self.err()),
			Ok(_) => (),
		}

		self._chunk = self._chunk_buffer.chars().collect();
		self._index = 0;

		Ok(())
	}

	/// Proceed to the next character in the source text.
	fn advance(&mut self) -> ParseResult
	{
		if self.current() == None {
			self.next_line()
		}
		else {
			self._index += 1;
			Ok(())
		}
	}
	
	fn eat(&mut self, chars: &str) -> ParseResult
	{
		// TODO
		let mut chars = chars.chars();

		loop {
			let Some(required) = chars.next() else {
				return Ok(())
			};
			
			self.advance();
		}
	}
	
	fn eat_spaces(&mut self) -> ParseResult
	{
		while self.c == Some(' ') {
			self.advance()?;
		}

		Ok(())
	}
}


#[cfg(test)]
mod test
{
	use std::io::Cursor;

	use crate::parser::CharmParser;
	use crate::utils::*;

	#[test] fn test_eat()
	{
		let source = Cursor::new("test");
		let mut parser = CharmParser::init(source, &TEST_CONFIG);
		let r = parser.eat("test");
		assert_eq!(r, Ok(()));
	}
}
