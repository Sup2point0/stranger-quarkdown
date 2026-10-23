use super::out::*;
use super::wait::*;
use crate::colours::*;

use crossterm::{
	cursor,
	terminal::{ Clear, ClearType },
	execute,
};

use std::io;


macro_rules! step
{
	(
		$msg:literal $(, $($args:tt)* )?
	) => {
		{
			let msg = format!($msg $(, $($args)* )?);
			_step_(&msg, &msg)
		}
	};
	(
		before => $bef:literal $(, $($args1:tt)* )?;
		after  => $aft:literal $(, $($args2:tt)* )?;
	) => {
		{
			_step_(
				&format!($bef $(, $($args1)* )?),
				&format!($aft $(, $($args2)* )?),
			)
		}
	};
}
pub(crate) use step;


pub(crate) fn _step_(
	before: &str,
	after: &str,
) -> io::Result<()>
{
	let mut stdout = io::stdout();

	out!();
	out!("{Y}▸" => before);

	wait!()?;

	execute!(stdout, 
		cursor::MoveUp(2),
		Clear(ClearType::FromCursorDown),
		cursor::MoveToColumn(0),
	)?;
	out!("{GREY}{after}");

	Ok(())
}
