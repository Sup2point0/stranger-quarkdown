use super::out::*;
use super::wait::*;
use crate::colours::*;

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
}
pub(crate) use step;


pub(crate) fn _step_(
	before: &str,
	after: &str,
) -> io::Result<()>
{
	out!();
	out!("{Y}▸" => before);
	wait!()?;
	Ok(())
}
