use std::{
	fs::File,
	io::BufReader, io::BufRead,
	iter,
	str::Chars,
};

use super::{ BufferedParser, ParseResult, ParseError };
use crate::{ SquarkupConfig, FileData };


/// A parser for the 'squark charm' header of a file.
pub struct CharmParser<'l>
{
	config: &'l SquarkupConfig,
	
	/// The current character the parser is pointing to.
	current: Option<char>,
	
	/// Have we encountered a `<!-- #SQUARK live!` yet?
	///
	/// If so, this means the user intends for the file to be squarked up, and error checking should be stricter to catch mistakes on their end.
	is_live: bool,
	
	/// The backing buffer that reads from the target file.
	_reader: BufReader<File>,
	
	/// The currently in-memory chunk to process.
	_chunk: String,
	
	/// An iterator over the characters of the current chunk.
	_chars: Chars<'l>,
}

impl<'l> CharmParser<'l>
{
	pub fn init(file: File, config: &SquarkupConfig) -> Self
	{
		let mut out = Self {
			config,
			current: None,
			is_live: false,
			_reader: BufReader::new(file),
			_chunk: String::new(),
			_chars: "".chars(),
		}
		
		out.load_line();
		
		out
	}
	
	/// Run the parser to completion, extracting the metadata from the squark charm (if present) of the target file.
	pub fn parse(&mut self) -> Option<FileData>
	{
		match self._parse() {
			Ok(r) => Some(r),
			Err(ParseError::NoMatch) => None,
			Err(ParseError(e)) => {
				Log.err(e.to_string());
				None,
			},
		}
	}
	
	fn _parse(&mut self) -> Result<FileData, ParseError>
	{
		self.eat_spaces()?;
		
		let mut heading = None;
		
		if self.current == '#' {
			heading = Some(self.parse_heading()?);
		}
		
		self.parse_charm()?;
		
		Ok(FileData {
			heading,
		})
	}
	
	fn parse_heading(&mut self)
	{
		self.eat("#")
	}
	
	fn parse_charm(&mut self) -> ParseResult
	{
		
	}
}

impl<'l> BufferedParser for CharmParser<'l>
{
	/// Read the next line of the source text into memory.
	fn next_line(&mut self) -> ParseResult
	{
		self._reader.read_line(&mut self._chunk)?;
		self._chars = self._chunk.chars();
		self.current = self._chars.next();
	}

	/// Proceed to the next character in the source text.
	fn advance(&mut self) -> ParseResult
	{
		self.current = self.chars.next();
		
		if self.current == None {
			self.next_line();
		}
		
		Err(if self.is_live {
			ParseError::FatalEnd
		} else {
			ParseError::NoMatch
		})?
	}
	
	fn eat(&mut self, chars: impl Iterator<item = char>) -> ParseResult
	{
		loop {
			let Some(required) = chars.next() else { return };
			
			this.advance();
		}
	}
	
	fn eat_spaces(&mut self) -> ParseResult
	{
		while this.current == ' ' {
			this.advance()?;
		}
	}
}
