/// Core parser functionality not relevant to Squarkdown is split off into `parser_core`.
mod parser_core;


use tinyvec::{ TinyVec, tiny_vec };

use super::*;
use crate::{
	log, utils,
	FileData, SquarkupConfig,
	str,
};

use std::collections::HashMap;
use std::fs::File;
use std::io::{ BufRead, BufReader, Read };
use std::iter;
use std::assert_matches;


pub type FieldValues = TinyVec<[String; 4]>;


/// A parser for the charm squark of a file.
pub struct CharmParser<'l, Source: Read = File>
{
	/// Settings to use when resolving e.g. filepaths.
	config: &'l SquarkupConfig,

	/// Non-crashing errors to report to the user.
	errors: Vec<ParseError>,
	
	/// Have we encountered a `<!-- #SQUARK live!` yet?
	///
	/// If so, this means the user intends for the file to be squarked up, and error checking should be stricter to catch mistakes on their end.
	is_live: bool,

	/// Have we reached the end of the source?
	is_eof: bool,

	// == INTERNALS == //
	
	/// The backing buffer that reads from the target file.
	_reader: BufReader<Source>,

	/// The index in the current line the parser is pointing to.
	_index: usize,

	/* NOTE:
		Storing a `Chars` iterator over `_line_buffer` origind lifetime issues =(
		Having another `Vec<char>` is a little duplication, but it does make it much nicer to work with
	*/

	/// Individual characters of the currently in-memory line to process. Guaranteed to be terminated by a `\n` newline.
	_line: Vec<char>,
	
	/// The currently in-memory line to process.
	/// 
	/// This backs `._line`. Reuse this between line reads to avoid excessive allocations!
	_line_buffer: String,
}

/// The public parser interface.
impl<'l, Source: Read> CharmParser<'l, Source>
{
	/// Construct a parser for parsing the charm squark of `file`, using settings from `config`.
	pub fn init(file: Source, config: &'l SquarkupConfig) -> Result<Self, ParseError>
	{
		let mut out = Self {
			config,
			errors: vec![],
			is_live: false,
			is_eof: false,
			_reader: BufReader::new(file),
			_index: 0,
			_line: vec![],
			_line_buffer: String::new(),
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

		let heading = self._line[self._index..].iter().collect();
		Ok(utils::trim_end(heading))
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
		let origin = err_msg!("parsing charm squark flags");

		let mut flags = vec![];

		self.eat_spaces();

		while let Some(c) = self.current()
			&& c != '\n'
		{
			let ident = self.parse_ident()?;

			if self.current() == Some('!') {
				flags.push(ident);
			}
			else {
				self.errors.push(ParseError::MissingInput {
					origin: origin(),
					expected: format!("{ident}!"),
					actual: ident,
				})
			}
			
			self.advance(origin)?;
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
	fn parse_fields(&mut self) -> ParseResult<HashMap<String, FieldValues>>
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
	/// | field1 = value
	/// | field2 = value1 / value2
	///   ^^^^^^   ^^^^^^   ^^^^^^
	/// | field3 = value1 / value2 / value3
	/// -->
	/// ```
	fn parse_field(&mut self) -> ParseResult<(String, FieldValues)>
	{
		let ctx = err_msg!("parsing charm squark field");

		self.eat("|", ctx)?;
		let key = self.parse_ident()?;
		self.eat_whitespace();
		self.eat("=", ctx)?;
		self.eat_whitespace();

		loop {
			unimplemented!()
		}

		// (key, _)

		unimplemented!()
	}

	/// Parse 1 or more values in a charm squark field.
	/// 
	/// Stops when it reaches either either a `|` field separator or `-->` terminator.
	/// 
	/// ```ts
	/// <!-- #SQUARK live!
	/// | field =
	///     / value1
	///     / value2
	///       ^^^^^^
	/// | field = value
	/// -->
	/// ```
	fn parse_values(&mut self) -> ParseResult<FieldValues>
	{
		let origin = err_msg!("parsing values in charm squark field");

		/// All values collected so far.
		let mut values = tiny_vec!([String; 4]);

		/// The current value being built.
		let mut value = str!("");

		let mut can_terminate = false;

		self.eat_whitespace();

		while let Some(c) = self.current()
		{
			match c {
				// ` / ` flushes current value
				'/' if can_terminate && self.eat_whitespace() => {
					values.push(utils::trim_end(value.clone()));
					value.clear();
					can_terminate = false;
					continue;
				},

				// `|` terminates
				'|' if can_terminate => break,

				// `-->` terminates
				'-' if can_terminate && let Ok(_) = self.try_eat("-->") => break,

				_ => {
					can_terminate = utils::is_whitespace(c);
					
					if utils::is_whitespace(c) {
						value.push(' ');
					} else {
						value.push(c);
					}

					// normalise multiple whitespace into one ' '
					if can_terminate {
						self.eat_whitespace();
					} else {
						self.advance(origin)?
					}
				},
			}
		}
		
		if !value.is_empty() {
			values.push(utils::trim_end(value));
		}

		Ok(values)
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

	use tinyvec::tiny_vec;

	use crate::parser::*;
	use crate::utils::*;

	#[test] fn parse_heading_matches_single_line()
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

	#[test] fn parse_heading_matches_multi_line()
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

	#[test] fn parse_heading_fails()
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

	#[test] fn parse_flags_matches()
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
	
	#[test] fn parse_flags_fails()
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

	#[test] fn parse_values_single_usual()
	{
		test_exact(&[
			"success\n| field = value",
			"success\n-->",
		],
		|mut parser, _case| {
			let mut values = parser.parse_values().unwrap().into_iter();
			assert_eq!( values.next(), Some(str!("success")) );
		});
	}

	#[test] fn parse_values_single_multi_line()
	{
		test_exact(&[
			"suc\ncess |",
			"suc\n cess |",
			"suc \ncess |",
			"suc \n cess |",
		],
		|mut parser, _case| {
			let mut values = parser.parse_values().unwrap().into_iter();
			assert_eq!( values.next(), Some(str!("suc cess")) );
		});
	}

	#[test] fn parse_values_single_weird()
	{
		test_exact(&[
			"success\n  | field = value",
			"success \n| field = value",
			"success \n  | field = value",
		],
		|mut parser, _case| {
			let mut values = parser.parse_values().unwrap().into_iter();
			assert_eq!( values.next(), Some(str!("success")) );
		});
	}
}
