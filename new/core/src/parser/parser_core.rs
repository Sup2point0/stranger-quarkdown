use super::*;
use crate::core::*;
use crate::colours::*;
use crate::macros::*;

use std::iter;


/// Core parser internals, not specific to Squarkdown-Flavoured Markdown.
impl CharmParser<'_>
{
	/// Is the parser currently pointing outside the bounds of the current chunk?
	pub(super) fn is_out_of_bounds(&self) -> bool
	{
		self.i >= self.source.len()
	}

	/// The character in the source the parser is currently pointing to.
	/// 
	/// This returns `None` iff the parser has reached the end of its source and is out of bounds.
	pub(super) fn current(&self) -> Option<char>
	{
		self.source.get(self.i).copied()
	}

	/// Peek the next character in the line immediately after the current character.
	/// 
	/// Returns `None` if the parser is at the end of a line (since loading in the next line would require flushing the buffer).
	pub(super) fn peek(&self) -> Option<char>
	{
		self.source.get(self.i + 1).copied()
	}

	/// Get a preview of the upcoming text.
	pub(super) fn preview(&self) -> String
	{
		const PREVIEW_CHARS: usize = 20;

		let end = (self.i + PREVIEW_CHARS).min(self.source.len());
		let chars = self.source.get(self.i..end);
		
		match chars {
			Some(c) => c.iter().collect(),
			None => str!("END OF FILE"),
		}
	}

	/// Proceed to the next character in the source text.
	/// 
	/// Errors if the parser is out-of-bounds *before* advancing.
	/// 
	/// Skips `\r` characters.
	pub(super) fn advance(&mut self) -> SquarkResult
	{
		if self.is_out_of_bounds() {
			return self.err_eof();
		}

		self.i += 1;

		while let Some('\r') = self.current() {
			self.i += 1;
		}

		Ok(())
	}
	
	/// Consume exactly `target`, erroring on failure.
	pub(super) fn eat(&mut self,
		target: &str,
		to: impl Fn() -> String,
	) -> SquarkResult
	{
		for expected in target.chars()
		{
			match self.current() {
				Some(c) if c != expected => {
					return Err(SquarkError::Unrecoverable {
						msg: fmt!("unexpected input"),
						hint: fmt!("expected {target} to {}, but found: {}", to(), self.preview()),
						debug: self.show_ctx_stack(),
					});
				}
				None => return self.err_eof(),
				_ => (),
			}
			self.advance()?;
		}
		Ok(())
	}
	
	/// Attempt to consume exactly `target`. On failure, backtrack and return [`SquarkError::ABANDON`].
	pub(super) fn try_eat(&mut self, target: &str) -> SquarkResult
	{
		let init = self.i;

		for expected in target.chars()
		{
			if self.current() != Some(expected) {
				self.i = init;
				return Err(SquarkError::ABANDON);
			}
			self.advance()?;
		}

		Ok(())
	}
	
	/// Consume `target` disregarding casing, erroring on failure.
	pub(super) fn eat_caseless(&mut self,
		target: &str,
		to: impl Fn() -> String,
	) -> SquarkResult
	{
		for mut expected in target.chars()
		{
			expected.make_ascii_lowercase();

			match self.current() {
				Some(c) if c.to_ascii_lowercase() != expected => {
					return Err(SquarkError::Unrecoverable {
						msg: fmt!("unexpected input"),
						hint: fmt!("expected {target} to {}, but found: {}", to(), self.preview()),
						debug: self.show_ctx_stack(),
					});
				}
				None => return self.err_eof(),
				_ => (),
			};
			
			self.advance()?;
		}
		Ok(())
	}

	/// Attempt to consume `target` disregarding casing. On failure, backtrack and return [`SquarkError::ABANDON`]
	pub(super) fn try_eat_caseless(&mut self, target: &str) -> SquarkResult
	{
		let init = self.i;

		for mut expected in target.chars()
		{
			expected.make_ascii_lowercase();

			if self.current().map(|c| c.to_ascii_lowercase()) != Some(expected) {
				self.i = init;
				return Err(SquarkError::ABANDON);
			}
			self.advance()?;
		}

		Ok(())
	}
	
	/// Consume 0 or more space characters. Returns `true` if any characters were consumed.
	pub(super) fn eat_spaces(&mut self) -> bool
	{
		let mut did_consume = false;

		while self.current() == Some(' ') {
			let _ = self.advance();
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
			let _ = self.advance();
			did_consume = true;
		}

		did_consume
	}

	/// Parse an identifier like `sup`, `sup-world`, `internal.flag`.
	/// 
	/// Identifiers cannot start with `-`.
	pub(super) fn parse_ident(&mut self) -> SquarkResult<String>
	{
		ctx!(self, ParseCtx::IDENT =>
		{
			// FIXME require at least 1 character
			let mut chars = vec![];

			if let Some('-') = self.current() {
				return Err(SquarkError::Unrecoverable {
					msg: fmt!("illegal input: {}", self.preview()),
					hint: fmt!("identifiers cannot start with {W}'-'"),
					debug: self.show_ctx_stack(),
				});
			}

			if let Some(c) = self.current()
				&& !matches!(c, 'a'..='z' | 'A'..='Z' | '0'..='9' | '-' | '_' | '.')
			{
				return Err(SquarkError::Unrecoverable {
					msg: fmt!("expected identifier, but found: {W}{}", self.preview()),
					hint: fmt!("identifiers cannot start with {W}{c:?}"),
					debug: self.show_ctx_stack(),
				});
			}

			while let Some(c) = self.current()
				&& matches!(c, 'a'..='z' | 'A'..='Z' | '0'..='9' | '-' | '_' | '.')
			{
				chars.push(c);
				self.advance()?;
			}
			
			Ok(chars.into_iter().collect())
		})
	}
}

impl CharmParser<'_>
{
	/// Return the appropriate response for an unexpected end of file.
	/// 
	/// If `live!` has been found already, this is critical since the user intended for Squarkdown to squarkup the file.
	/// 
	/// If not, then Squarkdown can just ignore the file.
	pub(super) fn err_eof(&self) -> SquarkResult
	{
		let msg = str!("unexpected end of file");
		let hint = str!();
		let debug = self.show_ctx_stack();

		let err = if self.is_live {
			SquarkError::Unrecoverable { msg, hint, debug }
		} else {
			SquarkError::Recoverable { msg, hint, debug }
		};

		Err(err)
	}

	/// Build the debug diagnostics for printing errors.
	pub(super) fn show_ctx_stack(&self) -> Vec<String>
	{
		(
			iter::once(slash!("in: {GREY1}{}", self.filepath))
			.chain(self.ctx.stack().iter().rev().map(ToString::to_string))
		).collect()
	}
}


#[cfg(test)] use super::test_utils::*;
#[cfg(test)] use crate::utils::testing::*;

#[cfg(test)] use assertables::*;
	
	
#[test] fn advance_and_current()
{
	let mut parser = CharmParser::new("012\n345", TEST_FILE.clone(), &TEST_CONFIG);
	assert_eq!( parser.current(), Some('0') );
	assert_ok!( parser.advance() ); assert_eq!( parser.current(), Some('1') );
	assert_ok!( parser.advance() ); assert_eq!( parser.current(), Some('2') );
	assert_ok!( parser.advance() ); assert_eq!( parser.current(), Some('\n') );
	assert_ok!( parser.advance() ); assert_eq!( parser.current(), Some('3') );
	assert_ok!( parser.advance() ); assert_eq!( parser.current(), Some('4') );
	assert_ok!( parser.advance() ); assert_eq!( parser.current(), Some('5') );
	assert_ok!( parser.advance() ); assert_eq!( parser.current(), None );
	assert_err!( parser.advance() );
}

#[test] fn advance_and_peek()
{
	let mut parser = CharmParser::new("012\n345", TEST_FILE.clone(), &TEST_CONFIG);
	assert_eq!( parser.peek(), Some('1') );
	assert_ok!( parser.advance() ); assert_eq!( parser.peek(), Some('2') );
	assert_ok!( parser.advance() ); assert_eq!( parser.peek(), Some('\n') );
	assert_ok!( parser.advance() ); assert_eq!( parser.peek(), Some('3') );
	assert_ok!( parser.advance() ); assert_eq!( parser.peek(), Some('4') );
	assert_ok!( parser.advance() ); assert_eq!( parser.peek(), Some('5') );
	assert_ok!( parser.advance() ); assert_eq!( parser.peek(), None );
	assert_ok!( parser.advance() );
	assert_err!( parser.advance() );
}

#[test] fn preview()
{
	test_exact(&[
		"sup",
		"sup\n",
		"sup world",
		"sup world\n",
	],
	|parser, case| {
		assert_eq!( parser.preview(), case );
	});

	test_exact(&[
		"The quick brown fox jumps over the lazy dog",
		"The quick brown fox jumps over the lazy dog\n",
	],
	|parser, case| {
		assert_starts_with!( parser.preview(), case.chars().take(10).collect::<String>() );
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
		let r = parser.eat("FAIL", to!());
		assert_err!(r);
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
		assert_ok!( parser.eat(case, to!()) );
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
		assert_ok!( parser.eat_caseless(&case.to_ascii_uppercase(), to!()) );
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
			assert_ok!( parser.advance() );
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
		let r = parser.parse_ident();
		assert_ok!( &r );
		assert_eq!( r.unwrap(), case.to_string() );
	});
}
