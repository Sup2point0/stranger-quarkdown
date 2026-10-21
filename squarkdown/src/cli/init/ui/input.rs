use super::out::*;
use crate::colours::*;
use crate::macros::*;

use crossterm::{
	cursor,
	terminal::{ Clear, ClearType },
	execute,
};

use std::io::{ self, Write };


macro_rules! input
{
	(  $($args:tt)*  ) => {
		{
			ask!($($args)*);
			_input_()
		}
	}
}
pub(crate) use input;


pub(crate) fn _input_() -> io::Result<String>
{
	let mut stdout = io::stdout();
	let mut out = str!();

	print!(" {GREY}└  {G}");
	stdout.flush()?; 

	io::stdin().read_line(&mut out)?;

	execute!(stdout,
		cursor::MoveUp(1),
		Clear(ClearType::FromCursorDown),
		cursor::MoveToColumn(0),
	)?;

	out!("{GREY}{out}");

	Ok(out)
}
