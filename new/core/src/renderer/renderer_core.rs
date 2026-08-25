use super::*;
use crate::{
	SquarkResult, SquarkError,
	macros::*,
};

use std::io::{ Read, Write, BufRead };


impl<Source: Read, Target: Write>
	Renderer<Source, Target>
{
	pub(super) fn is_past_end_of_line(&self) -> bool
	{
		self._index >= self._line.len()
	}

	pub(super) fn current(&self) -> Option<char>
	{
		self._line.get(self._index).copied()
	}
	
	pub(super) fn peek(&self) -> Option<char>
	{
		self._line.get(self._index + 1).copied()
	}

	pub(super) fn preview(&self) -> &str {
		todo!()
	}

	pub(super) fn next_line(&mut self) -> SquarkResult
	{
		if self.is_done {
			return Err(todo!());
		}

		match self._reader.read_line(&mut self._line_buffer) {
			Err(..) => return Err(todo!()),
			Ok(0) => self.is_done = true,
			Ok(..) => (),
		}

		if self._line.last() != Some(&'\n') {
			self._line.push('\n');
		}

		self._index = 0;

		Ok(())
	}

	pub(super) fn advance(&mut self) -> SquarkResult
	{
		self._index += 1;

		if self.is_past_end_of_line() {
			self.next_line()
		} else {
			Ok(())
		}
	}
	
	pub(super) fn try_eat(&mut self, sequence: &str) -> SquarkResult<bool> {
		todo!()
	}

	pub(super) fn emit(&mut self, content: &str) -> SquarkResult
	{
		match self._writer.write(content.as_bytes())
		{
			Ok(n) => Ok(()),
			Ok(0) => Err(SquarkError::Unrecoverable {
				msg: str!(slash!("could not write to {}", &self.filepath.expect("renderer should know where it's rendering to"))),
				hint: str!("this may mean the file was deleted, moved or locked mid-write"),
				debug: vec![
					fmt!("tried to write `{content}`"),
				],
			}),
			// TODO retry on interruption
			Err(e) => Err(SquarkError::External {
				err: bx!(e),
				msg: str!(slash!("could not write to {}", &self.filepath.expect("renderer should know where it's rendering to"))),
			}),
		}
	}
}
