use utf8_chars::BufReadCharsExt;

use super::*;
use crate::{
	SquarkResult, SquarkError,
	macros::*,
};

use std::io::{ Read, Write, BufRead };


/// The number of characters the renderer reads into a chunk at a time.
pub(super) const CHUNK_SIZE: usize = 128;


impl<Source: Read, Target: Write>
	Renderer<Source, Target>
{
	pub(super) fn has_exhausted_chunk(&self) -> bool
	{
		self._index >= self._chunk.len()
	}

	/// The character in the source the renderer is currently pointing to.
	/// 
	/// This returns `None` iff the renderer has reached the end of its source and is out of bounds.
	pub(super) fn current(&self) -> Option<char>
	{
		self._chunk.get(self._index).copied()
	}

	pub(super) fn next_chunk(&mut self) -> SquarkResult
	{
		debug_assert!(!self.is_done);

		let chunk = self._reader.chars().take(CHUNK_SIZE);
		let mut t = 0;

		for (i, c) in chunk.enumerate() {
			t += 1;

			match c {
				Ok(c) => self._chunk[i] = c,
				Err(e) => return Err(SquarkError::External {
					err: bx!(e),
					msg: str!(slash!("could not read from {}", self.source_filepath)),
				}),
			}
		}

		if t == 0 {
			self.is_done = true;
		}

		self._chunk.truncate(t);
		self._index = 0;

		Ok(())
	}

	pub(super) fn advance(&mut self) -> SquarkResult
	{
		self._advance_()?;

		if self.current() == Some('\\') {
			self._advance_()?;
			self._advance_()?;
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
				msg: str!(slash!("could not write to {}", self.source_filepath)),
				hint: str!("this may mean the file was deleted, moved or locked mid-write"),
				debug: vec![
					fmt!("tried to write `{content}`"),
				],
			}),
			Ok(..) => Ok(()),
			// TODO retry on interruption
			Err(e) => Err(SquarkError::External {
				err: bx!(e),
				msg: str!(slash!("could not write to {}", self.source_filepath)),
			}),
		}
	}
}
