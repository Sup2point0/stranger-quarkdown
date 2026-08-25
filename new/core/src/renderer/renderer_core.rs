use super::*;
use crate::{
	SquarkResult,
};

use std::io::{ Read, Write };


impl<Source: Read, Target: Write>
	Renderer<Source, Target>
{
	pub(super) fn current(&self) -> Option<char> {
		todo!()
	}
	
	pub(super) fn peek(&self) -> Option<char> {
		todo!()
	}

	pub(super) fn preview(&self) -> &str {
		todo!()
	}

	pub(super) fn next_line(&mut self) -> SquarkResult {
		todo!()
	}

	pub(super) fn advance(&mut self) -> SquarkResult {
		todo!()
	}
	
	pub(super) fn try_eat(&mut self, sequence: &str) -> SquarkResult<bool> {
		todo!()
	}

	pub(super) fn emit(&mut self) {
		todo!()
	}
}
