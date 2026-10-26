use super::*;
use crate::prelude::*;
use crate::utils::colours::*;
use crate::utils::macros::*;


/// Core parser internals, not specific to Squarkdown-Flavoured Markdown.
impl CharmParser<'_>
{
	/// The current line of source text to process.
	/// 
	/// Errors if the parser is out of bounds.
	pub(super) fn line(&self) -> SquarkResult<&[char]>
	{
		Ok(
			&self.source_lines.get(self.l)
				.ok_or_else(|| self.err_eof())?
		)
	}

	/// The current character in the current line to process.
	/// 
	/// This returns `\n` if the parser is at the end of the current line.
	/// 
	/// Errors if the parser is out of bounds.
	pub(super) fn current(&self) -> SquarkResult<char>
	{
		Ok(
			self.line()?
				.get(self.i)
				.copied()
				.unwrap_or('\n')
		)
	}

	/// Peek the next character in the source immediately after the current character.
	/// 
	/// If the current character is at the end of a line, this returns `\n` instead of the first character of the next line.
	/// 
	/// Errors if the parser is out of bounds.
	pub(super) fn peek(&self) -> Option<char>
	{
		let line = self.line().ok()?;
		let i = self.i + 1;

		if i < line.len() {
			line.get(i).copied()
		}
		else if i == line.len() {
			Some('\n')
		}
		else {
			self.peek_line()?.first().copied()
		}
	}

	pub(super) fn peek_line(&self) -> Option<&[char]>
	{
		Some(
			&self.source_lines.get(self.l + 1)?
		)
	}

	/// Get a preview of the upcoming text.
	pub(super) fn preview(&self) -> String
	{
		match self.line()
		{
			Ok(line) => line[self.i..].into_iter().collect(),
			Err(..) => str!("END OF FILE"),
		}
	}

	/// Proceed to the next character in the source text.
	/// 
	/// Errors if the parser is out-of-bounds *before* advancing.
	/// 
	/// Skips `\r` characters.
	pub(super) fn advance(&mut self) -> SquarkResult
	{
		let line = self.line()?;

		if self.i >= line.len() {
			self.advance_line()?;
		} else {
			self.i += 1;
		}

		Ok(())
	}

	pub(super) fn advance_line(&mut self) -> SquarkResult
	{
		if self.l >= self.source_lines.len() {
			return Err(self.err_eof())
		}

		self.l += 1;
		self.i = 0;

		Ok(())
	}

	/// Check that `needle` is upcoming without consuming.
	/// 
	/// This does not check across line breaks, and returns `false` instead of erroring if lookahead goes out-of-bounds.
	/// 
	/// `needle` must be ASCII-only since this directly uses byte indices.
	pub(super) fn lookahead(&self, needle: &str) -> bool
	{
		let Ok(line) = self.line()
			else { return false };
		
		needle.char_indices().all(|(i, c)| line.get(self.i + i) == Some(&c))
	}
}

/// Consumption
impl CharmParser<'_>
{
	/// Consume exactly `target`, erroring on failure.
	pub(super) fn eat(&mut self,
		target: &str,
		to: impl Fn() -> String,
	) -> SquarkResult
	{
		for expected in target.chars()
		{
			if self.current()? != expected {
				return Err(SquarkError::Recoverable {
					msg: fmt!("unexpected input"),
					hint: fmt!("expected {W}{BOLD}{target}{UNBOLD} {G}{}, but found: {W}{}", to(), self.preview()),
					debug: self.ctx.printed(),
				});
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
			if self.current()? != expected {
				self.i = init;
				return Err(SquarkError::ABANDON)
			}
			self.advance()?;
		}

		Ok(())
	}
	
	/// Consume `target` disregarding casing, erroring on failure.
	#[allow(unused)]
	pub(super) fn eat_caseless(&mut self,
		target: &str,
		to: impl Fn() -> String,
	) -> SquarkResult
	{
		for expected in target.chars()
		{
			if !self.current()?.eq_ignore_ascii_case(&expected) {
				return Err(SquarkError::Recoverable {
					msg: fmt!("unexpected input"),
					hint: fmt!("expected {target} to {}, but found: {}", to(), self.preview()),
					debug: self.ctx.printed(),
				});
			}
			self.advance()?;
		}
		Ok(())
	}

	/// Attempt to consume `target` disregarding casing. On failure, backtrack and return [`SquarkError::ABANDON`]
	pub(super) fn try_eat_caseless(&mut self, target: &str) -> SquarkResult
	{
		let init = self.i;

		for expected in target.chars()
		{
			if !self.current()?.eq_ignore_ascii_case(&expected) {
				self.i = init;
				return Err(SquarkError::ABANDON)
			}
			self.advance()?;
		}

		Ok(())
	}
	
	/// Consume 0 or more space characters. Returns `true` if any characters were consumed.
	pub(super) fn eat_spaces(&mut self) -> bool
	{
		let mut did_consume = false;

		while let Ok(' ') = self.current() {
			let _ = self.advance();
			did_consume = true;
		}

		did_consume
	}

	/// Consume 0 more whitespace characters, including tabs and newlines. Returns `true` if any characters were consumed.
	pub(super) fn eat_whitespace(&mut self) -> bool
	{
		let mut did_consume = false;

		while self.current().is_ok_and(char::is_whitespace)
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
			let mut out = str!();

			if let Ok('-') = self.current() {
				return Err(SquarkError::Recoverable {
					msg: fmt!("illegal identifier: {}", self.preview()),
					hint: fmt!("identifiers cannot start with {W}-"),
					debug: self.ctx.printed(),
				});
			}

			if let Ok(c) = self.current() && !Self::is_ident_char(c) {
				return Err(SquarkError::Recoverable {
					msg: fmt!("expected identifier, but found: {W}{}", self.preview()),
					hint: fmt!("identifiers cannot start with {W}{c}"),
					debug: self.ctx.printed(),
				});
			}

			while let Ok(c) = self.current() && Self::is_ident_char(c) {
				out.push(c);
				self.advance()?;
			}

			Ok(out)
		})
	}
}

/// Shared uilities
impl CharmParser<'_>
{
	/// Is `c` a valid character for starting an identifier?
	pub(super) fn is_ident_char(c: char) -> bool
	{
		matches!(c, 'a'..='z' | 'A'..='Z' | '0'..='9' | '-' | '_' | '.' | '~')
	}

	/// Return the appropriate response for an unexpected end of file.
	/// 
	/// If `live!` has been found already, this is critical since the user intended for Squarkdown to squarkup the file.
	/// 
	/// If not, then Squarkdown can just ignore the file.
	pub(super) fn err_eof(&self) -> SquarkError
	{
		let msg = str!("unexpected end of file");
		let hint = str!();
		let debug =
			[
				self.source_printed(zero!(self.l - 1), 0, usize::MAX),
				self.ctx.printed(),
			].concat();

		if self.is_live {
			SquarkError::Unrecoverable { msg, hint, debug }
		} else {
			SquarkError::Recoverable { msg, hint, debug }
		}
	}
}

/// Debug
impl CharmParser<'_>
{
	/// Produce the lines of a snapshot of the source text, focused around `from..=to` in `line`.
	/// 
	/// If `from` or `to` are out-of-bounds, they are clamped.
	/// 
	/// # Panics
	/// 
	/// - If `line` is out-of-bounds.
	pub(super) fn source_printed(&self,
		line_idx: usize,
		mut from: usize,
		mut to: usize,
	) -> Vec<String>
	{
		let mut out = Vec::with_capacity(3);
		out.push(str!(" "));

		/* NOTE: When displaying line numbers, add 1 because `line-idx` is zero-indexed! */

		// previous line
		if line_idx > 0 {
			out.push(fmt!(
				"{GREY1}{:6} {GREY}│  {W}{}",
				(line_idx - 1) + 1,
				self.source_lines[line_idx - 1].iter().collect::<String>(),
			));
		}

		// focus lines
		let line = &self.source_lines[line_idx];

		if from != to
		{
			from = from.min(line.len());
			to   = to.min(line.len());
			let focus: String = line[from..to].iter().collect();

			let r_before = 0..from;
			let r_after  = to..line.len();
			let before = if r_before.is_empty() {str!("")} else {line[r_before].iter().collect()};
			let after  = if r_after.is_empty()  {str!("")} else {line[r_after].iter().collect()};

			out.push(fmt!(
				"{GREY1}{:6} {GREY}│  {W}{before}{R}{BOLD}{focus}{UNBOLD}{W}{after}",
				line_idx + 1,
			));
		}
		else {
			out.push(fmt!(
				"{GREY1}{:6} {GREY}│  {W}{}",
				line_idx + 1,
				line.iter().collect::<String>(),
			));
			out.push(fmt!(
				"{:6} {GREY}│  {:width$}{R}{BOLD}^^^{UNBOLD}",
				"",
				"",
				width = from,
			));
		}

		// next line
		if let Some(next_line) = self.source_lines.get(line_idx + 1) {
			out.push(fmt!(
				"{GREY}{:6} │  {W}{}",
				(line_idx + 1) + 1,
				next_line.iter().collect::<String>(),
			));
		} else {
			out.push(fmt!("{:6} {GREY}│  {GREY}<{BOLD}END OF FILE{UNBOLD}>", ""));
		}

		out.push(str!(" "));
		out
	}
}


// == TESTS == //

#[cfg(test)] use super::test_utils::*;
#[cfg(test)] use crate::utils::testing::*;

#[cfg(test)] use assertables::{ assert_ok, assert_err, assert_starts_with };
#[cfg(test)] use pretty_assertions::{ assert_eq };

#[cfg(test)] use std::assert_matches;


#[cfg(test)]
mod preview {
	use super::*;
	use super::assert_eq;

	#[test] fn basic()
	{
		test_exact(&[
			"sup",
			"sup world",
		],
		|parser, case| {
			assert_eq!( parser.preview(), case );
		});
	}

	#[test] fn cut_too_long() {
		test_exact(&[
			"The quick brown fox jumps over the lazy dog",
			"The quick brown fox jumps over the lazy dog\n",
		],
		|parser, case| {
			assert_starts_with!( parser.preview(), case.chars().take(10).collect::<String>() );
		});
	}

	#[test] fn cut_at_newline() {
		test_expected(&[
			("sup\nnope",       "sup"),
			("sup world\nnope", "sup world"),
		],
		|parser, &expected| {
			assert_eq!( parser.preview(), expected );
		});
	}
}

#[cfg(test)]
mod advance
{
	use super::*;

	#[test] fn and_current()
	{
		let mut parser = CharmParser::new("012\n345", TEST_FILE.clone(), &TEST_CONFIG);
		assert_matches!( parser.current(), Ok('0') );
		assert_ok!( parser.advance() ); assert_matches!( parser.current(), Ok('1') );
		assert_ok!( parser.advance() ); assert_matches!( parser.current(), Ok('2') );
		assert_ok!( parser.advance() ); assert_matches!( parser.current(), Ok('\n') );
		assert_ok!( parser.advance() ); assert_matches!( parser.current(), Ok('3') );
		assert_ok!( parser.advance() ); assert_matches!( parser.current(), Ok('4') );
		assert_ok!( parser.advance() ); assert_matches!( parser.current(), Ok('5') );
		assert_ok!( parser.advance() ); assert_matches!( parser.current(), Ok('\n') );
		assert_ok!( parser.advance() ); assert_err!( parser.current() );
		assert_err!( parser.advance() );
	}

	#[test] fn and_peek()
	{
		let mut parser = CharmParser::new("012\n345", TEST_FILE.clone(), &TEST_CONFIG);
		assert_matches!( parser.peek(), Some('1') );
		assert_ok!( parser.advance() ); assert_matches!( parser.peek(), Some('2') );
		assert_ok!( parser.advance() ); assert_matches!( parser.peek(), Some('\n') );
		assert_ok!( parser.advance() ); assert_matches!( parser.peek(), Some('3') );
		assert_ok!( parser.advance() ); assert_matches!( parser.peek(), Some('4') );
		assert_ok!( parser.advance() ); assert_matches!( parser.peek(), Some('5') );
		assert_ok!( parser.advance() ); assert_matches!( parser.peek(), Some('\n') );
		assert_ok!( parser.advance() ); assert_matches!( parser.peek(), None );
		assert_ok!( parser.advance() );
		assert_err!( parser.advance() );
	}
}

#[cfg(test)]
mod eat {
	use super::*;

	#[test] fn fails()
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

	#[test] fn matches()
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

#[cfg(test)]
mod eat_spaces {
	use super::*;
	use super::assert_eq;

	#[test] fn fails()
	{
		test_exact(&[
			"nothing",
			"nothing ",
			"n othing ",
		],
		|mut parser, _case| {
			assert_eq!( parser.eat_spaces(), false );
			assert_matches!( parser.current(), Ok('n') );
		});
	}

	#[test] fn stops()
	{
		test_exact(&[
			" stop",
			" stop ",
			"  stop ",
		],
		|mut parser, _case| {
			assert_eq!( parser.eat_spaces(), true );
			assert_matches!( parser.current(), Ok('s') );
		});
	}

	#[test] fn matches()
	{
		test_exact(&[
			" ",
			"  ",
			"        ",
		],
		|mut parser, _case| {
			assert_eq!( parser.eat_spaces(), true );
			assert_matches!( parser.current(), Ok('\n') );
		});
	}
}

#[cfg(test)]
mod eat_whitespace {
	use super::*;
	use super::assert_eq;

	#[test] fn stops()
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

			// FIXME
			// if parser.current() != Ok('s') {
			// 	assert_ok!( parser.advance() );
			// 	assert_matches!( parser.current(), Ok('s') );
			// }
		});
	}

	#[test] fn matches()
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
			assert_err!( parser.current() );
		});
	}
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
