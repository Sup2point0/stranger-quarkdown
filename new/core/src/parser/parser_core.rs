use super::*;
use crate::{
	macros::*,
};

use std::io::{ BufRead, Read };



/// Core parser internals, not specific to Squarkdown-Flavoured Markdown.
impl<Source: Read> CharmParser<Source>
{
	/// Is the parser currently pointing outside the bounds of the current chunk?
	pub(super) fn is_past_end_of_line(&self) -> bool
	{
		self._index >= self._line.len()
	}

	/// The character in the source the parser is currently pointing to.
	/// 
	/// This returns `None` iff the parser has reached the end of its source and is out of bounds.
	pub(super) fn current(&self) -> Option<char>
	{
		self._line.get(self._index).copied()
	}

	/// Peek the next character in the line immediately after the current character.
	/// 
	/// Returns `None` if the parser is at the end of a line (since loading in the next line would require flushing the buffer).
	pub(super) fn peek(&self) -> Option<char>
	{
		self._line.get(self._index + 1).copied()
	}

	/// Get a preview of the upcoming text.
	pub(super) fn preview(&self) -> String
	{
		const PREVIEW_CHARS: usize = 20;

		let end = (self._index + PREVIEW_CHARS).min(self._line.len());
		let chars = self._line.get(self._index..end);
		
		match chars {
			Some(c) => c.iter().collect(),
			None => str!("END OF FILE"),
		}
	}

	/// Read the next line of the source text into memory.
	/// 
	/// Errors if the parser has already reached the end of the source, or if reading from the buffer fails.
	pub(super) fn next_line(&mut self, when: impl Fn() -> String) -> ParseResult
	{
		if self.is_eof {
			return self.err_eof(when);
		}

		self._line_buffer.clear();

		match self._reader.read_line(&mut self._line_buffer) {
			Err(_) => return self.err_eof(when),
			Ok(0) => self.is_eof = true,
			Ok(_) => (),
		}

		self._line = self._line_buffer.chars().collect();

		/* NOTE: We rely on `\n` as the indicator of a new line, so even the last line must have one */
		if self._line.last() != Some(&'\n') {
			self._line.push('\n');
		}

		self._index = 0;

		Ok(())
	}

	/// Proceed to the next character in the source text, and read in a new line afterwards if necessary.
	/// 
	/// If this function is called when `self.is_eof: true`, this returns an end-of-input error.
	pub(super) fn advance(&mut self, when: impl Fn() -> String) -> ParseResult
	{
		self._index += 1;

		if self.is_past_end_of_line() {
			self.next_line(when)
		} else {
			Ok(())
		}
	}
	
	/// Consume exactly `target`, erroring on failure.
	pub(super) fn eat(&mut self,
		target: &str,
		to: impl Fn() -> String,
		when: impl Fn() -> String,
	) -> ParseResult
	{
		for expected in target.chars()
		{
			match self.current() {
				Some(c) if c != expected => {
					return Err(ParseFailure::UnexpectedInput {
						when: when(),
						expected: format!("{} to {}", target, to()),
						actual: self.preview(),
					});
				}
				None => return self.err_eof(when),
				_ => (),
			}
			self.advance(&when)?;
		}

		Ok(())
	}
	
	/// Attempt to consume exactly `target`. On failure, backtrack and return `NO_MATCH`.
	pub(super) fn try_eat(&mut self, target: &str) -> Recoverable
	{
		let init = self._index;

		for expected in target.chars()
		{
			if self.current() != Some(expected) {
				self._index = init;
				return Err(ParseFailure::NO_MATCH);
			}
			self.advance(when!())?;
		}

		Ok(())
	}
	
	/// Consume `target` disregarding casing, erroring on failure.
	pub(super) fn eat_caseless(&mut self,
		target: &str,
		to: impl Fn() -> String,
		when: impl Fn() -> String,
	) -> ParseResult
	{
		for mut expected in target.chars()
		{
			expected.make_ascii_lowercase();

			match self.current() {
				Some(c) if c.to_ascii_lowercase() != expected => {
					return Err(ParseFailure::UnexpectedInput {
						when: when(),
						expected: format!("{} {}", target, to()),
						actual: self.preview(),
					});
				}
				None => return self.err_eof(when),
				_ => (),
			};
			
			self.advance(&when)?;
		}

		Ok(())
	}

	/// Attempt to consume `target` disregarding casing, returning `NO_MATCH` on failure.
	pub(super) fn try_eat_caseless(&mut self, target: &str) -> Recoverable
	{
		let init = self._index;

		for mut expected in target.chars()
		{
			expected.make_ascii_lowercase();

			if self.current().map(|c| c.to_ascii_lowercase()) != Some(expected) {
				self._index = init;
				return Err(ParseFailure::NO_MATCH);
			}
			self.advance(when!())?;
		}

		Ok(())
	}
	
	/// Consume 0 or more space characters. Returns `true` if any characters were consumed.
	pub(super) fn eat_spaces(&mut self) -> bool
	{
		let mut did_consume = false;

		while self.current() == Some(' ') {
			let _ = self.advance(when!());  // safe due to loop check
			did_consume = true;
		}

		did_consume
	}

	/// Consume 0 more whitespace characters, including tabs and newlines. Returns `true` if any characters were consumed.
	pub(super) fn eat_whitespace(&mut self) -> bool
	{
		let mut did_consume = false;

		while let Some(c) = self.current()
			&& matches!(c, ' ' | '\t' | '\n')
		{
			let _ = self.advance(when!());  // safe due to loop check
			did_consume = true;
		}

		did_consume
	}

	/// Parse an identifier like `sup`, `sup-world`, `internal.flag`.
	/// 
	/// Identifiers cannot start with `-`.
	pub(super) fn parse_ident(&mut self, when: impl Fn() -> String) -> ParseResult<String>
	{
		// FIXME require at least 1 character
		let mut chars = vec![];

		if let Some('-') = self.current() {
			return Err(ParseFailure::IllegalInput {
				when: when(),
				because: str!("identifiers cannot start with `-`"),
				found: self.preview(),
			});
		}

		while let Some(c) = self.current()
			&& matches!(c, 'a'..='z' | 'A'..='Z' | '0'..='9' | '-' | '_' | '.')
		{
			chars.push(c);
			let _ = self.advance(when!("parsing an identifier"));
		}

		Ok(chars.into_iter().collect())
	}
}


#[cfg(test)]
mod test
{
	use std::assert_matches;
	use std::io::Cursor;

	use crate::parser::*;

	#[test] fn advance_and_current_single_line()
	{
		let cursor = Cursor::new("012345");
		let mut parser = CharmParser::init(cursor, None).unwrap();

		assert_eq!( parser.current(), Some('0') );
		assert_eq!( parser.advance(when!()), Ok(()) ); assert_eq!( parser.current(), Some('1') );
		assert_eq!( parser.advance(when!()), Ok(()) ); assert_eq!( parser.current(), Some('2') );
		assert_eq!( parser.advance(when!()), Ok(()) ); assert_eq!( parser.current(), Some('3') );
		assert_eq!( parser.advance(when!()), Ok(()) ); assert_eq!( parser.current(), Some('4') );
		assert_eq!( parser.advance(when!()), Ok(()) ); assert_eq!( parser.current(), Some('5') );
		assert_eq!( parser.advance(when!()), Ok(()) ); assert_eq!( parser.current(), Some('\n') );
		assert_eq!( parser.advance(when!()), Ok(()) ); assert_eq!( parser.current(), Some('\n') );
		assert_eq!( parser.advance(when!()), Err(ParseFailure::NO_MATCH) );
	}
	
	#[test] fn advance_and_current_multi_line()
	{
		let cursor = Cursor::new("012\n345");
		let mut parser = CharmParser::init(cursor, None).unwrap();

		assert_eq!( parser.current(), Some('0') );
		assert_eq!( parser.advance(when!()), Ok(()) ); assert_eq!( parser.current(), Some('1') );
		assert_eq!( parser.advance(when!()), Ok(()) ); assert_eq!( parser.current(), Some('2') );
		assert_eq!( parser.advance(when!()), Ok(()) ); assert_eq!( parser.current(), Some('\n') );
		assert_eq!( parser.advance(when!()), Ok(()) ); assert_eq!( parser.current(), Some('3') );
		assert_eq!( parser.advance(when!()), Ok(()) ); assert_eq!( parser.current(), Some('4') );
		assert_eq!( parser.advance(when!()), Ok(()) ); assert_eq!( parser.current(), Some('5') );
		assert_eq!( parser.advance(when!()), Ok(()) ); assert_eq!( parser.current(), Some('\n') );
		assert_eq!( parser.advance(when!()), Ok(()) ); assert_eq!( parser.current(), Some('\n') );
		assert_eq!( parser.advance(when!()), Err(ParseFailure::NO_MATCH) );
	}

	#[test] fn advance_and_peek_single_line()
	{
		let cursor = Cursor::new("012345");
		let mut parser = CharmParser::init(cursor, None).unwrap();

		assert_eq!( parser.peek(), Some('1') );
		assert_eq!( parser.advance(when!()), Ok(()) ); assert_eq!( parser.peek(), Some('2') );
		assert_eq!( parser.advance(when!()), Ok(()) ); assert_eq!( parser.peek(), Some('3') );
		assert_eq!( parser.advance(when!()), Ok(()) ); assert_eq!( parser.peek(), Some('4') );
		assert_eq!( parser.advance(when!()), Ok(()) ); assert_eq!( parser.peek(), Some('5') );
		assert_eq!( parser.advance(when!()), Ok(()) ); assert_eq!( parser.peek(), Some('\n') );
		assert_eq!( parser.advance(when!()), Ok(()) ); assert_eq!( parser.peek(), None );
		assert_eq!( parser.advance(when!()), Ok(()) ); assert_eq!( parser.peek(), None );
		assert_eq!( parser.advance(when!()), Err(ParseFailure::NO_MATCH) );
	}

	#[test] fn advance_and_peek_multi_line()
	{
		let cursor = Cursor::new("012\n345");
		let mut parser = CharmParser::init(cursor, None).unwrap();

		assert_eq!( parser.peek(), Some('1') );
		assert_eq!( parser.advance(when!()), Ok(()) ); assert_eq!( parser.peek(), Some('2') );
		assert_eq!( parser.advance(when!()), Ok(()) ); assert_eq!( parser.peek(), Some('\n') );
		assert_eq!( parser.advance(when!()), Ok(()) ); assert_eq!( parser.peek(), None );
		assert_eq!( parser.advance(when!()), Ok(()) ); assert_eq!( parser.peek(), Some('4') );
		assert_eq!( parser.advance(when!()), Ok(()) ); assert_eq!( parser.peek(), Some('5') );
		assert_eq!( parser.advance(when!()), Ok(()) ); assert_eq!( parser.peek(), Some('\n') );
		assert_eq!( parser.advance(when!()), Ok(()) ); assert_eq!( parser.peek(), None );
		assert_eq!( parser.advance(when!()), Ok(()) ); assert_eq!( parser.peek(), None );
		assert_eq!( parser.advance(when!()), Err(ParseFailure::NO_MATCH) );
	}

	#[test] fn preview()
	{
		test_exact(&[
			"Sup",
			"Sup World",
		],
		|parser, case| {
			assert_eq!( parser.preview(), case.to_string() + "\n" );
		});
	}

	#[test] fn next_line()
	{
		test_expected(&[
			(
				"# Sup\n<!-- #SQUARK live! -->\n\nSup, World!",
				["# Sup\n", "<!-- #SQUARK live! -->\n", "\n", "Sup, World!\n"],
			)
		],
		|mut parser, _expected| {
			for line in _expected {
				assert_eq!( parser._line, line.chars().collect::<Vec<_>>() );
				let _ = parser.next_line(when!());
			}
		});
	}

	#[test] fn eat_fails()
	{
		test_exact(&[
			" ",
			"test",
			"testing testing",
			"testing 123",
		],
		|mut parser, _case| {
			assert_matches!( parser.eat("FAIL", when!(), when!()), Err(ParseFailure::UnexpectedInput{..}) );
		});
	}

	#[test] fn eat_matches()
	{
		test_exact(&[
			" ",
			"test",
			"testing testing",
			"testing 123",
		],
		|mut parser, case| {
			assert_eq!( parser.eat(case, when!(), when!()), Ok(()) );
		});
	}

	#[test] fn eat_caseless_matches()
	{
		test_exact(&[
			" ",
			"Test",
			"Testing TESTING",
			"tEsTiNg 123",
		],
		|mut parser, case| {
			assert_eq!( parser.eat_caseless(&case.to_ascii_uppercase(), when!(), when!()), Ok(()) );
		});
	}

	#[test] fn eat_spaces_fails()
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

	#[test] fn eat_spaces_stops()
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

	#[test] fn eat_spaces_matches()
	{
		test_exact(&[
			" ",
			"  ",
			"        ",
		],
		|mut parser, _case| {
			assert_eq!( parser.eat_spaces(), true );
			assert_eq!( parser.current(), Some('\n') );
		});
	}

	#[test] fn eat_whitespace_stops()
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
				assert_eq!( parser.advance(when!()), Ok(()) );
				assert_eq!( parser.current(), Some('s') );
			}
		});
	}

	#[test] fn eat_whitespace_matches()
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

	#[test] fn parse_ident()
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
			assert_eq!( parser.parse_ident(when!()), Ok(case.to_string()) );
		});
	}
}
