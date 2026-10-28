use super::out::*;
use crate::utils;
use crate::utils::colours::*;
use crate::utils::macros::*;

use crossterm::{
	cursor,
	terminal::{ Clear, ClearType },
	execute,
};

use std::fmt::Display;
use std::io::{ self, Write };


/// `:: str -> Result (Option String)`
macro_rules! input
{
	(
		before : $bef:literal $(, $($args1:tt)*)? ;
		after  : $aft:literal $(, $($args2:tt)*)? ;
	) => {
		_input_(
			format_args!($bef $(, $($args1)*)?),
			format_args!($aft $(, $($args2)*)?),
		)
	};
	
	(
		$msg:literal $(, $($args:tt)*)?
	) => {
		input!(
			before: $msg $(, $($args)*)?;
			after:  $msg $(, $($args)*)?;
		)
	};
}
pub(crate) use input;


pub(crate) fn _input_(
	before: impl Display,
	after: impl Display,
) -> io::Result<Option<String>>
{
	let mut stdout = io::stdout();
	
	ask!("{before}");
	print!(" {F}└  {G}");
	stdout.flush()?; 
	
	let mut out = str!();
	io::stdin().read_line(&mut out)?;
	out = utils::trim_end(out);

	execute!(stdout,
		cursor::MoveUp(2),
		Clear(ClearType::FromCursorDown),
		cursor::MoveToColumn(0),
	)?;

	if out.is_empty() {
		out!("{F}×" => "{after}");
		out!("{R}{ITAL}skipped{UNITAL}");
		Ok(None)
	} else {
		out!("{C}✓" => "{after}");
		out!("{F}{out}");
		Ok(Some(out))
	}
}
