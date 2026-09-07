use utf8_chars::BufReadCharsExt;

use super::*;
use crate::{
	SquarkResult, SquarkError,
	macros::*,
};

use std::io::{ Read, Write };


/// The number of characters the renderer reads into a chunk at a time.
pub(super) const CHUNK_SIZE: usize = 256;


impl<Source: Read, Target: Write>
	Renderer<Source, Target>
{
	pub(super) fn has_exhausted_chunk(&self) -> bool
	{
		self._index >= self._window.len()
	}

	/// The character in the source the renderer is currently pointing to.
	/// 
	/// This returns `None` iff the renderer has reached the end of its source and is out of bounds.
	pub(super) fn current(&self) -> Option<char>
	{
		self._window.get(self._index).copied()
	}

	/// Peek the character directly *after* the current character in the source.
	/// 
	/// This may trigger a chunk read from the buffer.
	pub(super) fn peek(&mut self) -> Option<char>
	{
		if self.is_done {
			None
		}
		else {
			if self._index == self._window.len() - 1 {
				let _ = self.next_chunk();
			}
			self._window.get(self._index + 1).copied()
		}
	}

	pub(super) fn preview(&self) -> String
	{
		const PREVIEW_CHARS: usize = 10;

		let end = (self._index + PREVIEW_CHARS).min(self._window.len());
		let chars = self._window.get(self._index..end);

		match chars {
			Some(c) => c.iter().collect(),
			None => str!("⏎"),
		}
	}

	/// Chop off the part of the `._window` that has already been processed.
	pub(super) fn cleanup_chunk(&mut self)
	{
		if self._index >= CHUNK_SIZE {
			self._window.drain(..self._index);
			self._index = 0;
		}
	}

	/// Append the next chunk of source text into `._window`.
	pub(super) fn next_chunk(&mut self) -> SquarkResult
	{
		debug_assert!(!self.is_done);

		let mut t = 0;

		for c in self._reader.chars().take(CHUNK_SIZE) {
			t += 1;

			match c {
				Ok(c) => self._window.push(c),
				Err(e) => return Err(SquarkError::External {
					err: bx!(e),
					msg: str!(slash!("could not read from: {}", self.source_filepath)),
				}),
			}
		}

		if t == 0 {
			self.is_done = true;
		}

		Ok(())
	}

	pub(super) fn advance(&mut self) -> SquarkResult
	{
		self._advance_()?;

		if self.current() == Some('\n') {
			self.line_number += 1;
			// self.emit(&fmt!("{:?}", self.ctx.stack()))?;
		}

		Ok(())
	}

	fn _advance_(&mut self) -> SquarkResult
	{
		self._index += 1;

		if self.has_exhausted_chunk() {
			self.next_chunk()
		} else {
			Ok(())
		}
	}

	pub(super) fn eat(&mut self,
		target: &str,
		to: impl Fn() -> String,
		hint: impl Fn() -> String,
	) -> SquarkResult
	{
		for expected in target.chars()
		{
			if self.current() != Some(expected) {
				return Err(SquarkError::Recoverable {
					msg: fmt!("expected {target} to {}", to()),
					hint: hint(),
					debug: vec![
						slash!("in {}:{}", self.source_filepath, self.line_number),
					],
				});
			}
			self.advance()?;
		}

		Ok(())
	}
	
	/// Attempt to consume exactly `target`, returning `Ok(true)` if succesful.
	pub(super) fn try_eat(&mut self, target: &str) -> SquarkResult<bool>
	{
		let init = self._index;

		for expected in target.chars()
		{
			if self.current() != Some(expected) {
				self._index = init;
				return Ok(false);
			}
			self.advance()?;
		}

		Ok(true)
	}

	/// Attempt to consume `target` disregarding casing, returning `true` if successful.
	pub(super) fn try_eat_caseless(&mut self, target: &str) -> SquarkResult<bool>
	{
		let init = self._index;

		for expected in target.chars() {
			if let Some(c) = self.current()
				&& c.to_ascii_lowercase() != expected.to_ascii_lowercase()
			{
				self._index = init;
				return Ok(false);
			}

			self.advance()?;
		}

		Ok(true)
	}

	/// Consume 0 or more whitespace characters, which includes tabs and newlines.
	pub(super) fn eat_whitespace(&mut self) -> SquarkResult<bool>
	{
		let mut did_consume = false;

		while let Some(c) = self.current()
			&& matches!(c, ' ' | '\t' | '\n')
		{
			self.advance()?;
			did_consume = true;
		}

		Ok(did_consume)
	}
}

impl<Source: Read, Target: Write>
	Renderer<Source, Target>
{
	/// Attempt to consume a ` #SQUARK <squark><?|.> -->` instance, pushing or popping the context stack as required.
	/// 
	/// `allow_open` must be enabled to consume `squark?`, and `allow_close` must be enabled to consume `squark.`. At least 1 of the 2 should be `true`.
	pub(super) fn try_eat_twin_squark(&mut self,
		squark: &str,
		ctx: Ctx,
		allow_open: bool,
		allow_close: bool,
		require_terminator: bool,
	) -> SquarkResult<bool>
	{
		let init = self._index;

		'abort: {
			self.eat_whitespace()?;
			if !self.try_eat_caseless("#SQUARK")? { break 'abort; }
			self.eat_whitespace()?;
			if !self.try_eat_caseless(squark)? { break 'abort; }

			if allow_open && self.try_eat("?")? {
				self.ctx.pop(Ctx::COMMENT);
				self.ctx.push(ctx);
			}
			else if allow_close && self.try_eat(".")? {
				self.ctx.pop(ctx);
			}
			else {
				// TODO colour
				self.errors.push(SquarkError::Recoverable {
					msg:  fmt!("unknown squark: `#SQUARK {squark}{}`", self.preview()),
					hint: str!("twin squarks should end in `?` to open a section, or `.` to close it"),
					debug: vec![
						slash!("in file: {}:{}", self.target_filepath, self.line_number),
					],
				});
			}

			// TODO allow excess input
			self.eat_whitespace()?;

			if require_terminator {
				self.eat("-->",
					to!("terminate squark"),
					hints!("close a slashed section like `<!-- #SQUARK slash. -->`"),
				)?;
			}

			return Ok(true)
		}

		self._index = init;
		Ok(false)
	}

	pub(super) fn try_open_close_squark(&mut self, squark: &str, ctx: Ctx) -> SquarkResult<bool> {
		self.try_eat_twin_squark(squark, ctx, true, true, true)
	}

	pub(super) fn try_open_squark(&mut self, squark: &str, ctx: Ctx) -> SquarkResult<bool> {
		self.try_eat_twin_squark(squark, ctx, true, false, true)
	}

	pub(super) fn try_close_squark(&mut self, squark: &str, ctx: Ctx) -> SquarkResult<bool> {
		self.try_eat_twin_squark(squark, ctx, false, true, true)
	}
}

impl<Source: Read, Target: Write>
	Renderer<Source, Target>
{
	pub(super) fn emit_char(&mut self, c: char) -> SquarkResult
	{
		let mut bytes = [0 as u8; 4];
		let encoded = c.encode_utf8(&mut bytes);

		self.emit(encoded)
	}

	pub(super) fn emit(&mut self, content: &str) -> SquarkResult
	{
		match self._writer.write(content.as_bytes())
		{
			Ok(0) => Err(SquarkError::Unrecoverable {
				msg: str!(slash!("could not write to: {}", self.target_filepath)),
				hint: str!("this may mean the file was deleted, moved or locked mid-write"),
				debug: vec![
					fmt!("tried to write `{content}`"),
				],
			}),
			Ok(..) => Ok(()),
			// TODO retry on interruption
			Err(e) => Err(SquarkError::External {
				err: bx!(e),
				msg: str!(slash!("could not write to: {}", self.target_filepath)),
			}),
		}
	}
}
