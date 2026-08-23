/// Core parser functionality not relevant to Squarkdown is split off into `parser_core`.
mod parser_core;


use std::{
	collections::HashMap,
	fs::File,
	io::{ BufRead, BufReader, Read },
	iter,
	assert_matches,
};

use super::*;
use crate::{
	log,
	FileData, SquarkupConfig,
	types::SquarkValue,
	str,
};


type Recoverable = ParseResult;


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
		Storing a `Chars` iterator over `_chunk_buffer` origind lifetime issues =(
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
		
		out.next_line(err_msg!("initialising parser"))?;
		
		Ok(out)
	}
	
	/// Run the parser to completion, extracting the metadata from the squark charm (if present) of the target file.
	pub fn parse(&mut self) -> Option<FileData>
	{
		match self._parse()
		{
			Ok(r) => Some(r),
			Err(ParseError::NO_MATCH) => None,
			Err(e) => {
				log::err(e);
				None
			},
		}
	}
}

/// Parser internals specialised to Squarkdown-Flavoured Markdown.
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
		self.eat("# ", err_msg!("parsing heading"))?;
		self.eat_spaces();

		Ok(self._chunk[self._index..].iter().collect())
	}
	
	/// Parse the `<!-- #SQUARK live! ... -->` charm squark.
	fn parse_charm_squark(&mut self) -> ParseResult<FileData>
	{
		self.try_parse_squark_live()?;
		let flags = self.parse_flags()?;
		let fields = self.parse_fields()?;

		Ok(FileData::init(flags, fields))
	}

	/// Attemp to look for `<!-- #SQUARK live!`.
	/// 
	/// If found, set `.is_live: true`; otherwise return `NO_MATCH`.
	fn try_parse_squark_live(&mut self) -> Recoverable
	{
		self.try_eat("<!--")?;
		self.eat_whitespace(); self.try_eat("#")?;
		self.eat_spaces(); self.try_eat_caseless("SQUARK")?;
		self.eat_spaces(); self.try_eat_caseless("live!")?;

		self.is_live = true;
		Ok(())
	}

	/// Parse the flags in the charm squark and return their identifiers.
	/// 
	/// ```ts
	/// <!-- #SQUARK live! feat! dev! -->
	///                    ^^^^  ^^^
	/// ```
	fn parse_flags(&mut self) -> ParseResult<Vec<String>>
	{
		let mut flags = vec![];

		loop {
			self.eat_spaces();
			if self.current() == None { break; }

			let ident = self.parse_ident()?;

			if self.current() == Some('!') {
				flags.push(ident);
				let _ = self.advance(err_msg!("parsing charm squark flags"));
			}
		}

		Ok(flags)
	}

	/// Parse the fields in the charm squark and return a hashmap of the data.
	/// 
	/// ```ts
	/// <!-- #SQUARK live!
	/// | field = value
	///   ^^^^^   ^^^^^
	/// | fields = value / value / value
	///   ^^^^^^   ^^^^^   ^^^^^   ^^^^^
	/// -->
	/// ```
	fn parse_fields(&mut self) -> ParseResult<HashMap<String, SquarkValue>>
	{
		let mut data = HashMap::new();

		loop {
			self.eat_whitespace();

			match self.current() {
				// done
				Some('-') => {
					self.eat("-->", err_msg!("parsing squark charm fields"))?;
					break;
				},

				// another field
				Some('|') => (),

				// bad
				Some(_) => Err(ParseError::UnexpectedInput {
					origin: str!("parsing squark charm fields"),
					expected: str!("`|` to start field in charm squark"),
					actual: self.preview(),
				})?,

				None => self.advance(err_msg!("parsing charm squark fields"))?,  // force error
			}

			let (key, value) = self.parse_field()?;
			data.insert(key, value);
		}

		Ok(data)
	}

	/// Parse a single field and return its key and value.
	/// 
	/// ```ts
	/// <!-- #SQUARK live!
	/// | field = value
	/// | field = value
	///   ^^^^^   ^^^^^
	/// | field = value
	/// -->
	/// ```
	fn parse_field(&mut self) -> ParseResult<(String, SquarkValue)>
	{
		// self.eat("|", "")?;

		unimplemented!()
	}

	/// Return the appropriate `Err(ParseError)` for an unexpected end of file.
	/// 
	/// If `live!` has been found already, this is critical since the user intended for Squarkdown to squarkup the file.
	/// 
	/// If not, then Squarkdown can just ignore the file.
	fn err_eof(&self, origin: impl Fn() -> String) -> ParseResult
	{
		Err(if self.is_live {
			ParseError::FatalEnd { origin: origin() }
		} else {
			ParseError::NO_MATCH
		})
	}
}


#[cfg(test)]
mod test
{
	use std::assert_matches;
	use std::io::Cursor;

	use crate::parser::*;
	use crate::utils::*;

	#[test] fn test_parse_heading_matches_single_line()
	{
		test_expected(&[
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
			assert_matches!( parser.parse_heading(), Err(ParseError::UnexpectedInput{..}) );
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
}
