use std::collections::HashMap;
use std::fs::File;
use std::io::{ BufRead, BufReader, Read };
use std::iter;
use std::assert_matches;

use super::*;


/// Core parser internals, not specific to Squarkdown-Flavoured Markdown.
impl<'l, Source: Read> CharmParser<'l, Source>
{
	/// Is the parser currently pointing outside the bounds of the current chunk?
	pub(super) fn is_past_end_of_line(&self) -> bool
	{
		self._index >= self._line.len()
	}

	/// The current character in the source the parser is pointing to, or `None` if it is out of bounds.
	pub(super) fn current(&self) -> Option<char>
	{
		self._line.get(self._index).copied()
	}

	/// Peek the next character in the line immediately after the current character.
	/// 
	/// Edge cases:
	/// 
	/// - `Some('\n')` if at the last character of a line
	/// - `None` if past the last character of a line
	pub(super) fn peek(&self) -> Option<char>
	{
		let next = self._line.get(self._index + 1);

		if next == None && self.current() != None {
			return Some('\n')
		} else {
			return next.copied();
		}
	}

	/// Get a preview of the upcoming text (for error messages).
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
	pub(super) fn next_line(&mut self, origin: impl Fn() -> String) -> ParseResult
	{
		self._line_buffer.clear();

		match self._reader.read_line(&mut self._line_buffer) {
			Ok(0) | Err(_) => return self.err_eof(origin),
			Ok(_) => (),
		}

		self._line = self._line_buffer.chars().collect();

		if self._line.last() == Some(&'\n') {
			self._line.pop();
		}

		self._index = 0;

		Ok(())
	}

	/// Proceed to the next character in the source text.
	/// 
	/// If we're at the end of the current line, this reads in a new line.
	pub(super) fn advance(&mut self, origin: impl Fn() -> String) -> ParseResult
	{
		if self.current() == None {
			self.next_line(origin)
		}
		else {
			self._index += 1;
			Ok(())
		}
	}
	
	/// Consume exactly `target`, erroring on failure.
	pub(super) fn eat(&mut self, target: &str, origin: impl Fn() -> String) -> ParseResult
	{
		for expected in target.chars()
		{
			match self.current() {
				Some(c) if c != expected => {
					return Err(ParseError::UnexpectedInput {
						origin: origin(),
						expected: target.to_string(),
						actual: self.preview(),
					});
				}
				None => return self.err_eof(origin),
				_ => (),
			}
			self.advance(&origin)?;
		}

		Ok(())
	}
	
	/// Attempt to consume exactly `target`, returning `NO_MATCH` on failure.
	pub(super) fn try_eat(&mut self, target: &str) -> Recoverable
	{
		for expected in target.chars()
		{
			if self.current() != Some(expected) {
				return Err(ParseError::NO_MATCH);
			}
			self.advance(err_msg!())?;
		}

		Ok(())
	}
	
	/// Consume `target` disregarding casing, erroring on failure.
	pub(super) fn eat_caseless(&mut self, target: &str, origin: impl Fn() -> String) -> ParseResult
	{
		for mut expected in target.chars()
		{
			expected.make_ascii_lowercase();

			match self.current() {
				Some(c) if c.to_ascii_lowercase() != expected => {
					return Err(ParseError::UnexpectedInput {
						origin: origin(),
						expected: target.to_string(),
						actual: self.preview(),
					});
				}
				None => return self.err_eof(origin),
				_ => (),
			};
			
			self.advance(&origin)?;
		}

		Ok(())
	}

	/// Attempt to consume `target` disregarding casing, returning `NO_MATCH` on failure.
	pub(super) fn try_eat_caseless(&mut self, target: &str) -> Recoverable
	{
		for mut expected in target.chars()
		{
			expected.make_ascii_lowercase();

			if self.current().map(|c| c.to_ascii_lowercase()) != Some(expected) {
				return Err(ParseError::NO_MATCH);
			}
			self.advance(err_msg!())?;
		}

		Ok(())
	}
	
	/// Consume 0 or more space characters. Returns `true` if any characters were consumed.
	pub(super) fn eat_spaces(&mut self) -> bool
	{
		let mut did_consume = false;

		while self.current() == Some(' ') {
			// safe due to loop check
			let _ = self.advance(err_msg!());
			did_consume = true;
		}

		did_consume
	}

	/// Consume 0 more whitespace characters, including tabs and newlines. Returns `true` if any characters were consumed.
	pub(super) fn eat_whitespace(&mut self) -> bool
	{
		let mut did_consume = false;

		loop {
			if self.current() == None {
				let r = self.advance(err_msg!());
				if r.is_err() { break; }
			}
			
			if let Some(c) = self.current()
			&& matches!(c, ' ' | '\t' | '\n')
			{
				// safe due to loop check
				let _ = self.advance(err_msg!());
				did_consume = true;
			}
			else {
				break;
			}
		}

		did_consume
	}

	/// Parse an identifier like `sup`, `sup-world`, `internal.flag`.
	pub(super) fn parse_ident(&mut self) -> ParseResult<String>
	{
		let mut chars = vec![];

		while let Some(c) = self.current()
			&& matches!(c, 'a'..='z' | 'A'..='Z' | '0'..='9' | '-' | '_' | '.')
		{
			chars.push(c);
			let _ = self.advance(err_msg!("parsing an identifier"));
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
	use crate::utils::*;

	#[test] fn advance_and_current_single_line()
	{
		let cursor = Cursor::new("012345");
		let mut parser = CharmParser::init(cursor, &TEST_CONFIG).unwrap();

		assert_eq!( parser.current(), Some('0') );
		assert!( parser.advance(err_msg!()).is_ok() ); assert_eq!( parser.current(), Some('1') );
		assert!( parser.advance(err_msg!()).is_ok() ); assert_eq!( parser.current(), Some('2') );
		assert!( parser.advance(err_msg!()).is_ok() ); assert_eq!( parser.current(), Some('3') );
		assert!( parser.advance(err_msg!()).is_ok() ); assert_eq!( parser.current(), Some('4') );
		assert!( parser.advance(err_msg!()).is_ok() ); assert_eq!( parser.current(), Some('5') );
		assert!( parser.advance(err_msg!()).is_ok() ); assert_eq!( parser.current(), None );
	}
	
	#[test] fn advance_and_current_multi_line()
	{
		let cursor = Cursor::new("012\n345");
		let mut parser = CharmParser::init(cursor, &TEST_CONFIG).unwrap();

		assert_eq!( parser.current(), Some('0') );
		assert!( parser.advance(err_msg!()).is_ok() ); assert_eq!( parser.current(), Some('1') );
		assert!( parser.advance(err_msg!()).is_ok() ); assert_eq!( parser.current(), Some('2') );
		assert!( parser.advance(err_msg!()).is_ok() ); assert_eq!( parser.current(), None );
		assert!( parser.advance(err_msg!()).is_ok() ); assert_eq!( parser.current(), Some('3') );
		assert!( parser.advance(err_msg!()).is_ok() ); assert_eq!( parser.current(), Some('4') );
		assert!( parser.advance(err_msg!()).is_ok() ); assert_eq!( parser.current(), Some('5') );
		assert!( parser.advance(err_msg!()).is_ok() ); assert_eq!( parser.current(), None );
	}

	#[test] fn advance_and_peek_single_line()
	{
		let cursor = Cursor::new("012345");
		let mut parser = CharmParser::init(cursor, &TEST_CONFIG).unwrap();

		assert_eq!( parser.peek(), Some('1') );
		assert!( parser.advance(err_msg!()).is_ok() ); assert_eq!( parser.peek(), Some('2') );
		assert!( parser.advance(err_msg!()).is_ok() ); assert_eq!( parser.peek(), Some('3') );
		assert!( parser.advance(err_msg!()).is_ok() ); assert_eq!( parser.peek(), Some('4') );
		assert!( parser.advance(err_msg!()).is_ok() ); assert_eq!( parser.peek(), Some('5') );
		assert!( parser.advance(err_msg!()).is_ok() ); assert_eq!( parser.peek(), Some('\n') );
		assert!( parser.advance(err_msg!()).is_ok() ); assert_eq!( parser.peek(), None );
	}

	#[test] fn advance_and_peek_multi_line()
	{
		let cursor = Cursor::new("012\n345");
		let mut parser = CharmParser::init(cursor, &TEST_CONFIG).unwrap();

		assert_eq!( parser.peek(), Some('1') );
		assert!( parser.advance(err_msg!()).is_ok() ); assert_eq!( parser.peek(), Some('2') );
		assert!( parser.advance(err_msg!()).is_ok() ); assert_eq!( parser.peek(), Some('\n') );
		assert!( parser.advance(err_msg!()).is_ok() ); assert_eq!( parser.peek(), None );
		assert!( parser.advance(err_msg!()).is_ok() ); assert_eq!( parser.peek(), Some('4') );
		assert!( parser.advance(err_msg!()).is_ok() ); assert_eq!( parser.peek(), Some('5') );
		assert!( parser.advance(err_msg!()).is_ok() ); assert_eq!( parser.peek(), Some('\n') );
		assert!( parser.advance(err_msg!()).is_ok() ); assert_eq!( parser.peek(), None );
	}

	#[test] fn preview()
	{
		test_exact(&[
			"Sup",
			"Sup World",
		],
		|parser, case| {
			assert_eq!( parser.preview(), case );
		});
	}

	#[test] fn next_line()
	{
		test_expected(&[
			(
				"# Sup\n<!-- #SQUARK live! -->\n\nSup, World!\n",
				["# Sup", "<!-- #SQUARK live! -->", "", "Sup, World!"],
			)
		],
		|mut parser, _expected| {
			for line in _expected {
				assert_eq!( parser._line, line.chars().collect::<Vec<_>>() );
				let _ = parser.next_line(err_msg!());
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
			assert_matches!( parser.eat("FAIL", err_msg!()), Err(ParseError::UnexpectedInput{..}) );
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
			assert_eq!( parser.eat(case, err_msg!()), Ok(()) );
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
			assert_eq!( parser.eat_caseless(&case.to_ascii_uppercase(), err_msg!()), Ok(()) );
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
			assert_eq!( parser.current(), None );
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
				assert_eq!( parser.advance(err_msg!()), Ok(()) );
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
			assert_eq!( parser.parse_ident(), Ok(case.to_string()) );
		});
	}
}
