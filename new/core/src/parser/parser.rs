use std::{
	fs::File,
	io::{ BufReader, BufRead, Read },
	iter,
	str::Chars,
};

use super::*;
use crate::{
	log,
	SquarkupConfig, FileData,
};


/// A parser for the charm squark of a file.
pub struct CharmParser<'l, Source: Read = File>
{
	/// Settings to use when resolving e.g. filepaths.
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
	/// Construct a parser for parsing the charm squark of `file`, using settings from `config`.
	pub fn init(file: Source, config: &'l SquarkupConfig) -> Result<Self, ParseError>
	{
		let mut out = Self {
			config,
			_reader: BufReader::new(file),
			_index: 0,
			_chunk: vec![],
			_chunk_buffer: String::new(),
			is_live: false,
		};
		
		out.next_line()?;
		
		Ok(out)
	}
	
	/// Run the parser to completion, extracting the metadata from the squark charm (if present) of the target file.
	pub fn parse(&mut self) -> Option<FileData>
	{
		match self._parse()
		{
			Ok(r) => Some(r),
			
			Err(ParseError::NoMatch) | Err(ParseError::NotLive) => None,
			
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
		self.eat_spaces();
		
		let mut heading = None;
		if self.current() == Some('#') {
			heading = Some(self.parse_heading()?);
		}

		self.eat_whitespace();

		let mut file_data = self.parse_charm_squark()?;

		if file_data.heading == None {
			file_data.heading = heading;
		}
		
		Ok(file_data)
	}
	
	/// Parse the `# Heading` element and extract the cleaned heading text.
	fn parse_heading(&mut self) -> ParseResult<String>
	{
		self.eat("#")?;
		self.eat_spaces();

		Ok(self._chunk[self._index..].iter().collect())
	}
	
	/// Parse the `<!-- #SQUARK live! ... -->` charm squark.
	fn parse_charm_squark(&mut self) -> ParseResult<FileData>
	{
		self.parse_squark_live()?;

		unimplemented!()
	}

	/// Look for `<!-- #SQUARK live!`, and if found set `.is_live: true`.
	fn parse_squark_live(&mut self) -> ParseResult
	{
		self.eat("<!--")?;
		self.eat_whitespace(); self.eat("#")?;
		self.eat_spaces(); self.eat_caseless("SQUARK")?;
		self.eat_spaces(); self.eat_caseless("live!")?;

		self.is_live = true;
		Ok(())
	}

	fn parse_flags(&mut self) -> ParseResult<Vec<String>>
	{
		let mut flags = vec![];

		loop {
			self.eat_spaces();
			if self.current() == None { break; }

			let ident = self.parse_ident()?;

			if self.current() == Some('!') {
				flags.push(ident);
				let _ = self.advance();
			}
		}

		Ok(flags)
	}

	fn err(&self) -> ParseError
	{
		if self.is_live {
			ParseError::FatalEnd
		} else {
			ParseError::NotLive
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
		self._chunk_buffer.clear();

		match self._reader.read_line(&mut self._chunk_buffer) {
			Ok(0) | Err(_) => return Err(self.err()),
			Ok(_) => (),
		}

		self._chunk = self._chunk_buffer.chars().collect();

		if self._chunk.last() == Some(&'\n') {
			self._chunk.pop();
		}

		self._index = 0;

		Ok(())
	}

	/// Proceed to the next character in the source text.
	/// 
	/// If we're at the end of the current line, this reads in a new line.
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
	
	/// Consume exactly `target`.
	fn eat(&mut self, target: &str) -> ParseResult
	{
		let mut chars = target.chars();

		loop {
			let Some(expected) = chars.next() else {
				return Ok(());
			};

			let Some(found) = self.current() else {
				return Err(ParseError::NoMatch);
			};

			if found != expected {
				return Err(ParseError::NoMatch);
			}
			
			self.advance()?;
		}
	}
	
	/// Consume `target`, without considering casing for letters.
	fn eat_caseless(&mut self, target: &str) -> ParseResult
	{
		let normalised = target.to_ascii_lowercase();
		let mut chars = normalised.chars();

		loop {
			let Some(expected) = chars.next() else {
				return Ok(());
			};

			let Some(found) = self.current() else {
				return Err(ParseError::NoMatch);
			};

			if found.to_ascii_lowercase() != expected {
				return Err(ParseError::NoMatch);
			}
			
			self.advance()?;
		}
	}
	
	/// Consume 0 or more space characters. Returns `true` if any characters were consumed.
	fn eat_spaces(&mut self) -> bool
	{
		let mut did_consume = false;

		while self.current() == Some(' ') {
			let _ = self.advance();
			did_consume = true;
		}

		did_consume
	}

	/// Consume 0 more whitespace characters, including tabs and newlines. Returns `true` if any characters were consumed.
	fn eat_whitespace(&mut self) -> bool
	{
		let mut did_consume = false;

		loop {
			if self.current() == None {
				let r = self.advance();
				if r.is_err() { break; }
			}
			
			if let Some(c) = self.current()
			&& matches!(c, ' ' | '\t' | '\n')
			{
				let _ = self.advance();
				did_consume = true;
			}
			else {
				break;
			}
		}

		did_consume
	}

	/// Parse an identifier like `sup`, `sup-world`, `internal.flag`.
	fn parse_ident(&mut self) -> ParseResult<String>
	{
		let mut chars = vec![];

		while let Some(c) = self.current()
			&& matches!(c, 'a'..'z' | 'A'..'Z' | '0'..'9' | '-' | '_' | '.')
		{
			chars.push(c);
			let _ = self.advance();
		}

		Ok(chars.into_iter().collect())
	}
}


#[cfg(test)]
mod test
{
	use std::io::Cursor;

	use crate::parser::*;
	use crate::utils::*;

	fn test_exact(cases: &[&'static str], test: impl Fn(CharmParser<Cursor<&&str>>, &str))
	{
		for case in cases {
			let cursor = Cursor::new(case);
			let parser = CharmParser::init(cursor, &TEST_CONFIG).unwrap();

			test(parser, case)
		}
	}

	fn test_expected<X>(
		cases: &[(&'static str, X)],
		test: impl Fn(CharmParser<Cursor<&&str>>, &X),
	)
	{
		for (source, expected) in cases {
			let cursor = Cursor::new(source);
			let parser = CharmParser::init(cursor, &TEST_CONFIG).unwrap();

			test(parser, expected)
		}
	}

	#[test] fn test_parse_heading_matches_single_line()
	{
		test_expected(&[
			("#",             ""),
			("# ",            ""),
			("# Sup",         "Sup"),
			("# Suppety Sup", "Suppety Sup"),
		],
		|mut parser, expected| {
			assert_eq!( parser.parse_heading(), Ok(str!(*expected)) );
		});
	}

	#[test] fn test_parse_heading_matches_multi_line()
	{
		test_expected(&[
			("#\nDECOY",             ""),
			("# \nDECOY",            ""),
			("# Sup\nDECOY",         "Sup"),
			("# Suppety Sup\nDECOY", "Suppety Sup"),
		],
		|mut parser, expected| {
			assert_eq!( parser.parse_heading(), Ok(str!(*expected)) );
		});
	}

	#[test] fn test_parse_heading_fails()
	{
		test_exact(&[
			" ",
			"Sup",
			"Don't Do It",
		],
		|mut parser, _case| {
			assert_eq!( parser.parse_heading(), Err(ParseError::NoMatch) );
		});
	}

	#[test] fn test_parse_flags_matches()
	{
		test_expected(&[
			("live!", vec![str!("live")]),
			("one! two!", vec![str!("one"), str!("two")]),
			("kebab-case! snake_case!", vec![str!("kebab-case"), str!("snake_case")]),
		],
		|mut parser, expected_flags| {
			let flags = parser.parse_flags().unwrap();

			for (found, expected) in flags.into_iter().zip(expected_flags) {
				assert_eq!( found, *expected );
			}
		});
	}
	
	#[test] fn test_parse_flags_fails()
	{
		test_expected(&[
			("live! ignore", vec![str!("live")]),
			("live!\nignore", vec![str!("live")]),
			("live! \nignore", vec![str!("live")]),
			("live!\n ignore", vec![str!("live")]),
			("one! ignore two!", vec![str!("one"), str!("two")]),
			("kebab-case! ignore-me snake_case! ignore_me", vec![str!("kebab-case"), str!("snake_case")]),
		],
		|mut parser, expected_flags| {
			let flags = parser.parse_flags().unwrap();

			for (found, expected) in flags.into_iter().zip(expected_flags) {
				assert_eq!( found, *expected );
			}
		});
	}

	#[test] fn test_parse_ident()
	{
		test_exact(&[
			"identifier",
			"camelCase",
			"PascalCase",
			"kebab-case",
			"snake_case",
			"dot.case",
			"very-much_mixedCase",
		],
		|mut parser, case| {
			assert_eq!( parser.parse_ident(), Ok(case.to_string()) );
		});
	}

	#[test] fn test_eat_whitespace_matches()
	{
		test_exact(&[
			" ",
			"  ",
			"        ",
			" \n",
			"\n ",
			" \n ",
			" \n \n\n",
			"\t",
			" \n\t ",
		],
		|mut parser, _case| {
			assert_eq!( parser.eat_whitespace(), true );
			assert_eq!( parser.current(), None );
		});
	}

	#[test] fn test_eat_whitespace_stops()
	{
		test_exact(&[
			" stop",
			"  stop",
			"        stop",
			" \nstop",
			"\n stop",
			" \n stop",
			" \n \n\nstop",
			"\tstop",
			" \n\t stop",
		],
		|mut parser, _case| {
			assert_eq!( parser.eat_whitespace(), true );

			if parser.current() != Some('s') {
				assert_eq!( parser.advance(), Ok(()) );
				assert_eq!( parser.current(), Some('s') );
			}
		});
	}

	#[test] fn test_eat_spaces_matches()
	{
		test_exact(&[
			" ",
			"  ",
			"        ",
		],
		|mut parser, _case| {
			assert_eq!( parser.eat_spaces(), true );
			assert_eq!( parser.current(), None );
		});
	}

	#[test] fn test_eat_spaces_stops()
	{
		test_exact(&[
			" stop",
			" stop ",
			"  stop ",
		],
		|mut parser, _case| {
			assert_eq!( parser.eat_spaces(), true );
			assert_eq!( parser.current(), Some('s') );
		});
	}

	#[test] fn test_eat_spaces_fails()
	{
		test_exact(&[
			"nothing",
			"nothing ",
			"n othing ",
		],
		|mut parser, _case| {
			assert_eq!( parser.eat_spaces(), false );
			assert_eq!( parser.current(), Some('n') );
		});
	}

	#[test] fn test_eat_whitespace()
	{
		test_exact(&[
			" ",
			"  ",
			"        ",
			" stop",
			" stop ",
			"nothing",
		],
		|mut parser, case| {
			assert_eq!( parser.eat(case), Ok(() ))
		});
	}

	#[test] fn test_eat_caseless_matches()
	{
		test_exact(&[
			" ",
			"Test",
			"Testing TESTING",
			"tEsTiNg 123",
		],
		|mut parser, case| {
			assert_eq!( parser.eat_caseless(&case.to_ascii_uppercase()), Ok(()) );
		});
	}

	#[test] fn test_eat_matches()
	{
		test_exact(&[
			" ",
			"test",
			"testing testing",
			"testing 123",
		],
		|mut parser, case| {
			assert_eq!( parser.eat(case), Ok(()) );
		});
	}

	#[test] fn test_eat_fails()
	{
		test_exact(&[
			" ",
			"test",
			"testing testing",
			"testing 123",
		],
		|mut parser, _case| {
			assert_eq!( parser.eat("FAIL"), Err(ParseError::NoMatch) );
		});
	}

	#[test] fn test_next_line()
	{
		test_expected(&[
			(
				"# Sup\n<!-- #SQUARK live! -->\n\nSup, World!\n",
				["# Sup", "<!-- #SQUARK live! -->", "", "Sup, World!"],
			)
		],
		|mut parser, _expected| {
			for chunk in _expected {
				assert_eq!( parser._chunk, chunk.chars().collect::<Vec<_>>() );
				let _ = parser.next_line();
			}
		});
	}
}
