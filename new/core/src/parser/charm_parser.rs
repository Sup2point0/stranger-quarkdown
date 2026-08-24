/// Core parser functionality not relevant to Squarkdown is split off into `parser_core`.
mod parser_core;


use tinyvec::tiny_vec;

use super::*;
use crate::{
	types::*,
	utils,
	utils::log,
	utils::macros::*,
};

use std::collections::HashMap;
use std::fs::File;
use std::io::{ BufReader, Read };


/// A parser for the charm squark of a file.
pub struct CharmParser<'l, Source: Read = File>
{
	/// Settings to use when resolving e.g. filepaths.
	config: &'l SquarkupConfig,

	/// Non-crashing errors to report to the user.
	errors: Vec<ParseFailure>,
	
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
		Storing a `Chars` iterator over `_line_buffer` whend lifetime issues =(
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
	pub fn init(file: Source, config: &'l SquarkupConfig) -> Result<Self, ParseFailure>
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
		
		out.next_line(when!("initialising parser"))?;
		
		Ok(out)
	}
	
	/// Run the parser to completion, extracting the metadata from the charm squark (if present) of the target file.
	pub fn parse(&mut self) -> Option<FileData>
	{
		match self._parse()
		{
			Ok(Ok(file_data)) => Some(file_data),
			Ok(Err(file_error)) => {
				log::bad!(file_error);
				None
			},
			Err(ParseFailure::NO_MATCH) => None,
			Err(parse_error) => {
				log::bad!(parse_error);
				None
			},
		}
	}
}

/// Parser internals specialised to Squarkdown-Flavoured Markdown.
impl<'l, Source: Read> CharmParser<'l, Source>
{
	fn _parse(&mut self) -> ParseResult<Result<FileData, FileError>>
	{
		self.eat_whitespace();
		
		let mut heading = None;
		if self.current() == Some('#') {
			heading = Some(self.parse_heading()?);
		}

		self.eat_whitespace();

		let (flags, fields) = self.parse_charm_squark()?;

		Ok(
			FileData::init(flags, fields, self.config).map(|mut f| {
				if f.heading == None {
					f.heading = heading;
				}
				return f;
			})
		)
	}
	
	/// Parse the `# Heading` element, extracting the cleaned heading text.
	fn parse_heading(&mut self) -> ParseResult<String>
	{
		self.eat("# ", to!("start page heading"), when!("parsing heading"))?;
		self.eat_spaces();

		let heading = self._line[self._index..].iter().collect();
		self.next_line(when!())?;  // safe cuz not live
		Ok(utils::trim_end(heading))
	}
	
	/// Parse the `<!-- #SQUARK live! ... -->` charm squark, extracting the flags and fields.
	fn parse_charm_squark(&mut self) -> ParseResult<(Strings, HashMap<String, Strings>)>
	{
		self.try_parse_squark_live()?;
		self.eat_spaces();
		let flags = self.parse_flags()?;
		self.eat_whitespace();
		let fields = self.parse_fields()?;

		Ok((flags, fields))
	}

	/// Attempt to look for `<!-- #SQUARK live!`.
	/// 
	/// If found, set `.is_live: true`; otherwise return `NO_MATCH`.
	fn try_parse_squark_live(&mut self) -> Recoverable
	{
		self.try_eat("<!--")?;
		self.eat_whitespace(); self.try_eat("#")?;
		self.eat_spaces(); self.try_eat_caseless("SQUARK")?;
		// TODO notify if live! not found
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
	fn parse_flags(&mut self) -> ParseResult<Strings>
	{
		let when = when!("parsing charm squark flags");

		let mut flags = strings!();

		while let Some(c) = self.current()
			&& c != '\n'
		{
			let ident = match self.parse_ident(when) {
				Ok(ident) => ident,

				/* NOTE:
					This means we've seen a `-` which starts the terminating `-->`.

					Or the user genuinely used an illegal identifier... maybe we can handle that properly in future
				*/
				Err(ParseFailure::IllegalInput{..}) => break,

				Err(e) => return Err(e),
			};

			if self.current() == Some('!') {
				flags.push(ident);
			}
			else {
				self.errors.push(ParseFailure::MissingInput {
					when: when(),
					expected: format!("{ident}! (flags must end in !)"),
					actual: ident,
				});
			}
			
			self.advance(when)?;
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
	fn parse_fields(&mut self) -> ParseResult<HashMap<String, Strings>>
	{
		let mut data = HashMap::new();

		self.eat_whitespace();

		while self.current() != Some('-') {
			let (key, value) = self.parse_field()?;
			data.insert(key, value);
			self.eat_whitespace();
		}
		
		self.eat("-->", to!("terminate charm squark"), when!("parsing charm squark fields"))?;

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
	fn parse_field(&mut self) -> ParseResult<(String, Strings)>
	{
		let when = when!("parsing charm squark field");

		self.eat("|", to!("start field in charm squark"), when)?;
		self.eat_whitespace();

		let key = self.parse_ident(when)?;

		self.eat_whitespace();
		self.eat("=", when!("after field identifier"), when)?;
		self.eat_whitespace();

		let values = self.parse_values()?;

		Ok((key, values))
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
	fn parse_values(&mut self) -> ParseResult<Strings>
	{
		let when = when!("parsing values in charm squark field");

		/// All values collected so far.
		let mut values = strings!();

		/// The current value being built.
		let mut value = str!("");

		/* NOTE: Start on `true`, so leading `/ ` is ignored */
		let mut can_terminate = true;

		self.eat_whitespace();

		while let Some(c) = self.current()
		{
			match c {
				// ` / ` flushes current value
				'/' if can_terminate && utils::is_whitespace(self.peek()
					.expect("safe from newline termination")) =>
				{
					let _ = self.advance(when!());  // safe from if check
					self.eat_whitespace();

					let trimmed = utils::trim_end(value.clone());
					if !trimmed.is_empty() {
						values.push(trimmed);
					}

					value.clear();
					continue;
				},

				// `|` terminates
				'|' if can_terminate => break,

				// `-->` terminates
				'-' if can_terminate && self.preview().starts_with("-->") => break,

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
						self.advance(when)?
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
	fn err_eof(&self, when: impl Fn() -> String) -> ParseResult
	{
		Err(if self.is_live {
			ParseFailure::FatalEnd { when: when() }
		} else {
			ParseFailure::NO_MATCH
		})
	}
}


#[cfg(test)]
mod test
{
	use tinyvec::tiny_vec;

	use crate::parser::*;
	use crate::utils::*;
	use crate::utils::macros::*;
	
	use std::collections::HashMap;
	use std::io::Cursor;
	use std::assert_matches;

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
			assert_matches!( parser.parse_heading(), Err(ParseFailure::UnexpectedInput{..}) );
		});
	}

	#[test] fn parse_charm_squark_no_fields()
	{
		let source = Cursor::new("<!-- #SQUARK live! -->");
		let mut parser = CharmParser::init(source, &TEST_CONFIG).unwrap();

		let (flags, fields) = parser.parse_charm_squark().unwrap();

		assert_eq!( flags, strings!() );
		assert_eq!( fields, HashMap::new() );
	}

	#[test] fn parse_charm_squark_one_field()
	{
		let source = Cursor::new("
<!-- #SQUARK live!
| dest = test
-->
		".trim());

		let mut parser = CharmParser::init(source, &TEST_CONFIG).unwrap();
		let (flags, fields) = parser.parse_charm_squark().unwrap();

		assert_eq!( flags, strings![] );

		assert_eq!( fields, HashMap::from([
			(str!("dest"), strings!["test"]),
		]))
	}

	#[test] fn parse_charm_squark_one_field_many_flags()
	{
		let source = Cursor::new("
<!-- #SQUARK live! feat! dev!
| dest = test
-->
		".trim());

		let mut parser = CharmParser::init(source, &TEST_CONFIG).unwrap();
		let (flags, fields) = parser.parse_charm_squark().unwrap();

		assert_eq!( flags, strings!["feat", "dev"] );

		assert_eq!( fields, HashMap::from([
			(str!("dest"), strings!["test"]),
		]))
	}

	#[test] fn parse_charm_squark_many_fields()
	{
		let source = Cursor::new("
<!-- #SQUARK live!
| dest = test
| head = tests
| title = testing
-->
		".trim());

		let mut parser = CharmParser::init(source, &TEST_CONFIG).unwrap();
		let (flags, fields) = parser.parse_charm_squark().unwrap();

		assert_eq!( flags, strings![] );
		
		assert_eq!( fields, HashMap::from([
			(str!("dest"), strings!["test"]),
			(str!("head"), strings!["tests"]),
			(str!("title"), strings!["testing"]),
		]))
	}

	#[test] fn parse_charm_squark_many_fields_values()
	{
		let source = Cursor::new("
<!-- #SQUARK live!
| dest = test
| tags = prot / deut / trit
-->
		".trim());

		let mut parser = CharmParser::init(source, &TEST_CONFIG).unwrap();
		let (flags, fields) = parser.parse_charm_squark().unwrap();

		assert_eq!( flags, strings![] );
		
		assert_eq!( fields, HashMap::from([
			(str!("dest"), strings!["test"]),
			(str!("tags"), strings!["prot", "deut", "trit"]),
		]))
	}

	#[test] fn parse_charm_squark_many_flags_fields_values()
	{
		let source = Cursor::new("
<!-- #SQUARK live! feat! dev!
| dest = test
| tags = prot / deut / trit
-->
		".trim());

		let mut parser = CharmParser::init(source, &TEST_CONFIG).unwrap();
		let (flags, fields) = parser.parse_charm_squark().unwrap();

		assert_eq!( flags, strings!["feat", "dev"] );
		
		assert_eq!( fields, HashMap::from([
			(str!("dest"), strings!["test"]),
			(str!("tags"), strings!["prot", "deut", "trit"]),
		]))
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

	#[test] fn parse_fields_usual()
	{
		let source = Cursor::new("
| field = value
| fields = one / two / three
-->
		".trim());

		let mut parser = CharmParser::init(source, &TEST_CONFIG).unwrap();
		let fields = parser.parse_fields().unwrap();

		assert!( fields.contains_key("field") );
		let field = &mut fields["field"].iter();
		assert_eq!( field.next(), Some(&str!("value")) );
		assert_eq!( field.next(), None );

		assert!( fields.contains_key("fields") );
		let field = &mut fields["fields"].iter();
		assert_eq!( field.next(), Some(&str!("one")) );
		assert_eq!( field.next(), Some(&str!("two")) );
		assert_eq!( field.next(), Some(&str!("three")) );
		assert_eq!( field.next(), None );
	}

	#[test] fn parse_field()
	{
		test_expected(&[
			("| field = value", ("field", vec!["value"])),
			("| field = one / two", ("field", vec!["one", "two"])),
			("| field = / one / two", ("field", vec!["one", "two"])),
		],
		|mut parser, (key, targets)| {
			let (field, values) = parser.parse_field().unwrap();
			assert_eq!( field, *key );

			for (left, right) in values.into_iter().zip(targets) {
				assert_eq!( left, *right );
			}
		});
	}

	#[test] fn parse_values_one_usual()
	{
		test_exact(&[
			"success\n| field = value",
			"success\n-->",
		],
		|mut parser, _case| {
			let mut values = parser.parse_values().unwrap().into_iter();
			assert_eq!( values.next(), Some(str!("success")) );
			assert_eq!( values.next(), None );
		});
	}

	#[test] fn parse_values_many_usual()
	{
		test_exact(&[
			"one / two / three\n| field = value",
			"one / two / three\n-->",
		],
		|mut parser, _case| {
			let mut values = parser.parse_values().unwrap().into_iter();
			assert_eq!( values.next(), Some(str!("one")) );
			assert_eq!( values.next(), Some(str!("two")) );
			assert_eq!( values.next(), Some(str!("three")) );
			assert_eq!( values.next(), None );
		});
	}

	#[test] fn parse_values_one_multi_line()
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
			assert_eq!( values.next(), None );
		});
	}

	#[test] fn parse_values_many_multi_line()
	{
		test_exact(&[
			"one\n / two\n / three |",
			"one\n/ two\n/ three |",
			"one / \ntwo / \nthree |",
			"one /\ntwo /\nthree |",
			"one\n / \ntwo\n / \nthree |",
			"one\n/\ntwo\n/\nthree |",
		],
		|mut parser, _case| {
			let mut values = parser.parse_values().unwrap().into_iter();
			assert_eq!( values.next(), Some(str!("one")) );
			assert_eq!( values.next(), Some(str!("two")) );
			assert_eq!( values.next(), Some(str!("three")) );
			assert_eq!( values.next(), None );
		});
	}

	#[test] fn parse_values_one_weird()
	{
		test_exact(&[
			"success\n  | field = value",
			"success \n| field = value",
			"success \n  | field = value",
		],
		|mut parser, _case| {
			let mut values = parser.parse_values().unwrap().into_iter();
			assert_eq!( values.next(), Some(str!("success")) );
			assert_eq!( values.next(), None );
		});
	}

	#[test] fn parse_values_one_bad()
	{
		test_expected(&[
			("not/good |", "not/good"),
			("not /good |", "not /good"),
			("not/ good |", "not/ good"),
		],
		|mut parser, expected| {
			let mut values = parser.parse_values().unwrap().into_iter();
			assert_eq!( values.next(), Some(str!(*expected)) );
			assert_eq!( values.next(), None );
		});
	}

	#[test] fn parse_values_many_weird()
	{
		test_exact(&[
			"one / two |",
			"one / / two |",
			"one / /\n/ two |",
		],
		|mut parser, _case| {
			let mut values = parser.parse_values().unwrap().into_iter();
			assert_eq!( values.next(), Some(str!("one")) );
			assert_eq!( values.next(), Some(str!("two")) );
			assert_eq!( values.next(), None );
		});
	}
}
