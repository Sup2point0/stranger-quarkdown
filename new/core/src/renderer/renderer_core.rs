use super::*;
use crate::{
	SquarkResult, SquarkError,
	macros::*,
};

use std::io::{ Read, Write, BufRead };


impl<Source: Read, Target: Write>
	Renderer<Source, Target>
{
	pub(super) fn has_exhausted_chunk(&self) -> bool
	{
		self._index >= self._chunk.len()
	}

	pub(super) fn current(&self) -> char
	{
		*self._chunk.get(self._index).expect("renderer's index should never be out of bounds while rendering")
	}

	pub(super) fn next_chunk(&mut self) -> SquarkResult
	{
		debug_assert!(!self.is_done);

		self._chunk_buffer.clear();

		match self._reader.read(&mut self._chunk_buffer) {
			Ok(0) => self.is_done = true,
			Ok(..) => (),
			Err(err) => return Err(SquarkError::External {
				err: bx!(err),
				msg: str!(slash!("could not read from {}", self.source_filepath)),
			}),
		}

		self._index = 0;

		Ok(())
	}

	pub(super) fn advance(&mut self) -> SquarkResult
	{
		self._advance_()?;

		if self.current() == '\\' {
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
			if self.current() != expected {
				self._index = init;
				return Ok(false);
			}
			self.advance()?;
		}

		Ok(true)
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
