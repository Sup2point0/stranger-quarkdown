use super::*;
use crate::core::*;
use crate::types::*;
use crate::utils;
use crate::colours::*;
use crate::macros::*;

use tinyvec::tiny_vec;

use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::{ Path, PathBuf };


// == PUBLIC == //

/// Parse the charm squark of the file at `filepath`, returning `Some(PageData)` for an active page, and `None` otherwise.
pub fn parse(
	filepath: impl AsRef<Path>,
	config: &SquarkupConfig,
) -> SquarkResult<Option<PageData>>
{
	// TODO read until -->
	let mut file = File::open(&filepath)?;
	let mut source = str!();
	file.read_to_string(&mut source)?;

	let parser = CharmParser::new(&source, filepath.as_ref().to_path_buf(), config);
	
	match parser.parse()
	{
		Ok(page) => Ok(Some(page)),
		Err(SquarkError::ABANDON) => Ok(None),
		Err(e) => Err(e)
	}
}


// == IMPLEMENTATION == //

/// A parser for the charm squark of a file.
pub struct CharmParser<'d>
{
	// == IMMUTABLE == //

	pub(super) config: &'d SquarkupConfig,

	/// The source file being parsed.
	pub(super) filepath: PathBuf,

	// == MUTABLE == //

	/// The source text to parse, containing the charm squark.
	pub(super) source: Vec<char>,

	/// The current index in [`Self::source`]'s characters.
	pub(super) i: usize,
	
	/// Have we encountered a `<!-- #SQUARK live!` yet?
	///
	/// If so, this means the user intends for the file to be squarked up, and error checking should be stricter to catch mistakes on their end.
	pub(super) is_live: bool,

	/// Accumulated errors during parsing.
	pub errors: SquarkError,

	/// Context stack of what the parser is doing, for error diagnostics.
	pub ctx: ContextStack<ParseCtx>,
}

/// The public parser interface.
impl<'d> CharmParser<'d>
{
	/// Construct a parser for parsing the charm squark of `file`, using settings from `config`.
	pub fn new(source: &str, filepath: PathBuf, config: &'d SquarkupConfig) -> Self
	{
		Self {
			config,
			filepath,
			source: source.chars().collect(),
			i: 0,
			is_live: false,
			errors: SquarkError::multiple(),
			ctx: ContextStack::new(),
		}
	}
	
	/// Run the parser to completion, extracting the heading and charm squark of the source.
	pub fn parse(mut self) -> SquarkResult<PageData>
	{
		self.eat_whitespace();
		
		let heading = {
			if self.current() == Some('#') {
				Some(self.parse_heading()?)
			}
			else {
				None
			}
		};

		self.eat_whitespace();

		let (flags, mut fields) = self.parse_charm_squark()?;
		fields.entry(str!("head")).or_insert(heading.into_iter().collect());

		let page = PageData::init(self.filepath.clone(), flags, fields, self.config)?;
		
		self.errors.or(page)
	}
}

/// Parser internals specialised to Squarkdown-Flavoured Markdown.
impl CharmParser<'_>
{
	/// Parse the `# Heading` element, extracting the cleaned heading text.
	pub(super) fn parse_heading(&mut self) -> SquarkResult<String>
	{
		ctx!(self, ParseCtx::HEADING =>
		{
			self.eat("#", to!("start heading"))?;

			while let Some('#') = self.current() {
				self.advance()?;
			}
			self.eat_spaces();

			let start = self.i;
			while let Some(c) = self.current() && c != '\n' {
				self.advance()?;
			}
			let stop = self.i;
			let heading = self.source[start..stop].iter().collect();
			
			Ok(utils::trim_end(heading))
		})
	}
	
	/// Parse the `<!-- #SQUARK live! ... -->` charm squark, extracting the flags and fields.
	pub(super) fn parse_charm_squark(&mut self) -> SquarkResult<(Strings, HashMap<String, Strings>)>
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
	/// If found, set `.is_live: true`.
	pub(super) fn try_parse_squark_live(&mut self) -> SquarkResult
	{
		self.try_eat("<!--")?;
		self.eat_whitespace(); self.try_eat_caseless("#SQUARK")?;
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
	pub(super) fn parse_flags(&mut self) -> SquarkResult<Strings>
	{
		ctx!(self, ParseCtx::FLAGS =>
		{
			let mut flags = strings!();

			while let Some(c) = self.current()
				&& c != '\n'
			{
				/* NOTE: We're assuming `-` starts the terminating `-->`, since identifiers can't start with `-`. However, it could be the user genuinely using an illegal identifier... maybe we can handle that properly in future. */
				if c == '-' { break; }

				let ident = self.parse_ident()?;

				if self.current() == Some('!') {
					flags.push(ident);
					self.advance()?;
				}
				else {
					self.errors.push(SquarkError::Recoverable {
						msg: fmt!("invalid flag: {}", self.preview()),
						hint: fmt!("flags must end in {W}!{G}, like: {W}{ident}!"),
						debug: self.show_ctx_stack(),
					});
					
					// TODO recover
				}
				
				self.eat_spaces();
			}

			Ok(flags)
		})
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
	pub(super) fn parse_fields(&mut self) -> SquarkResult<HashMap<String, Strings>>
	{
		ctx!(self, ParseCtx::FIELDS =>
		{
			let mut data = HashMap::new();

			self.eat_whitespace();

			while self.current() != Some('-') {
				let (key, value) = self.parse_field()?;
				data.insert(key, value);
				self.eat_whitespace();
			}
			
			self.eat("-->", to!("terminate charm squark"))?;

			Ok(data)
		})
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
	pub(super) fn parse_field(&mut self) -> SquarkResult<(String, Strings)>
	{
		ctx!(self, ParseCtx::FIELD =>
		{
			self.eat("|", to!("start field in charm squark"))?;
			self.eat_whitespace();

			let key = self.parse_ident()?;

			self.eat_whitespace();
			self.eat("=", to!("after field identifier"))?;
			self.eat_whitespace();

			let values = self.parse_values()?;

			Ok((key, values))
		})
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
	pub(super) fn parse_values(&mut self) -> SquarkResult<Strings>
	{
		ctx!(self, ParseCtx::VALUES =>
		{
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
						let _ = self.advance();  // safe from if check
						self.eat_whitespace();

						let trimmed = utils::trim_end(value.clone());
						if !trimmed.is_empty() {
							values.push(trimmed);
						}

						value.clear();
					}

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
							self.advance()?
						}
					},
				}
			}
			
			if !value.is_empty() {
				values.push(utils::trim_end(value));
			}

			Ok(values)
		})
	}
}


// == TESTS == //

#[cfg(test)] use super::test_utils::*;
#[cfg(test)] use crate::utils::testing::*;

#[cfg(test)] use assertables::*;
#[cfg(test)] use indoc::indoc;


#[cfg(test)]
mod full {
	use super::*;

	// TODO add tests

	#[test] fn parse_basic()
	{
		let source = indoc! {"
			# Test
			<!-- #SQUARK live!
			| dest = test
			-->
		"};

		let parser = CharmParser::new(source, TEST_FILE.clone(), &TEST_CONFIG);
		let file_data = parser.parse().unwrap();
		assert_eq!( file_data.heading, Some(str!("Test")) );
		assert_eq!( file_data.destination, dir!(TESTS / "src/routes/test") );
	}

	#[test] fn parse_basic_cr()
	{
		let source = indoc! {"
			# Test\r
			<!-- #SQUARK live!\r
			| dest = test\r
			-->
		"};

		let parser = CharmParser::new(source, TEST_FILE.clone(), &TEST_CONFIG);
		let file_data = parser.parse().unwrap();
		assert_eq!( file_data.heading, Some(str!("Test")) );
		assert_eq!( file_data.destination, dir!(TESTS / "src/routes/test") );
	}
}

#[cfg(test)]
mod partial {
	use super::*;

	#[test] fn parse_heading_matches_single_line()
	{
		test_expected(&[
			("# ",            ""),
			("# Sup",         "Sup"),
			("# Suppety Sup", "Suppety Sup"),
		],
		|mut parser, expected| {
			let r = parser.parse_heading();
			assert_ok!( &r );
			assert_eq!( r.unwrap(), str!(*expected) );
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
			let r = parser.parse_heading();
			assert_ok!( &r );
			assert_eq!( r.unwrap(), str!(*expected) );
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
			let r = parser.parse_heading();
			assert_err!( r );
			// TODO check error message
		});
	}

	#[test] fn parse_charm_squark_no_fields()
	{
		let source = "<!-- #SQUARK live! -->";
		let mut parser = CharmParser::new(source, TEST_FILE.clone(), &TEST_CONFIG);

		let (flags, fields) = parser.parse_charm_squark().unwrap();
		assert_eq!( flags, strings!() );
		assert_eq!( fields, HashMap::new() );
	}

	#[test] fn parse_charm_squark_one_field()
	{
		let source = indoc! {"
			<!-- #SQUARK live!
			| dest = test
			-->
		"};

		let mut parser = CharmParser::new(source, TEST_FILE.clone(), &TEST_CONFIG);
		let (flags, fields) = parser.parse_charm_squark().unwrap();

		assert_eq!( flags, strings![] );

		assert_eq!( fields, HashMap::from([
			(str!("dest"), strings!["test"]),
		]));
	}

	#[test] fn parse_charm_squark_one_field_many_flags()
	{
		let source = indoc! {"
			<!-- #SQUARK live! feat! dev!
			| dest = test
			-->
		"};

		let mut parser = CharmParser::new(source, TEST_FILE.clone(), &TEST_CONFIG);
		let (flags, fields) = parser.parse_charm_squark().unwrap();

		assert_eq!( flags, strings!["feat", "dev"] );

		assert_eq!( fields, HashMap::from([
			(str!("dest"), strings!["test"]),
		]));
	}

	#[test] fn parse_charm_squark_many_fields()
	{
		let source = indoc! {"
			<!-- #SQUARK live!
			| dest = test
			| head = tests
			| title = testing
			-->
		"};

		let mut parser = CharmParser::new(source, TEST_FILE.clone(), &TEST_CONFIG);
		let (flags, fields) = parser.parse_charm_squark().unwrap();

		assert_eq!( flags, strings![] );
		
		assert_eq!( fields, HashMap::from([
			(str!("dest"), strings!["test"]),
			(str!("head"), strings!["tests"]),
			(str!("title"), strings!["testing"]),
		]));
	}

	#[test] fn parse_charm_squark_many_fields_values()
	{
		let source = indoc! {"
			<!-- #SQUARK live!
			| dest = test
			| tags = prot / deut / trit
			-->
		"};

		let mut parser = CharmParser::new(source, TEST_FILE.clone(), &TEST_CONFIG);
		let (flags, fields) = parser.parse_charm_squark().unwrap();

		assert_eq!( flags, strings![] );
		
		assert_eq!( fields, HashMap::from([
			(str!("dest"), strings!["test"]),
			(str!("tags"), strings!["prot", "deut", "trit"]),
		]));
	}

	#[test] fn parse_charm_squark_many_flags_fields_values()
	{
		let source = indoc! {"
			<!-- #SQUARK live! feat! dev!
			| dest = test
			| tags = prot / deut / trit
			-->
		"};

		let mut parser = CharmParser::new(source, TEST_FILE.clone(), &TEST_CONFIG);
		let (flags, fields) = parser.parse_charm_squark().unwrap();

		assert_eq!( flags, strings!["feat", "dev"] );
		
		assert_eq!( fields, HashMap::from([
			(str!("dest"), strings!["test"]),
			(str!("tags"), strings!["prot", "deut", "trit"]),
		]));
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
			("live! ignore\n",     vec!["live"]),
			("live!\nignore\n",    vec!["live"]),
			("live! \nignore\n",   vec!["live"]),
			("live!\n ignore\n",   vec!["live"]),
			("one! ignore two!\n", vec!["one", "two"]),
			(
				"kebab-case! ignore-me snake_case! ignore_me\n",
				vec!["kebab-case", "snake_case"]
			),
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
		let source = indoc! {"
			| field = value
			| fields = one / two / three
			-->
		"};

		let mut parser = CharmParser::new(source, TEST_FILE.clone(), &TEST_CONFIG);
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
