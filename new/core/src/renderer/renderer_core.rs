use super::*;

use std::io::{ Read, Write };


impl<Source: Read, Target: Write>
	Renderer<Source, Target>
{
	pub(super) fn current(&mut self) {
		todo!()
	}
	
	pub(super) fn peek(&mut self) {
		todo!()
	}
	
	pub(super) fn try_eat(&mut self) {
		todo!()
	}
}
