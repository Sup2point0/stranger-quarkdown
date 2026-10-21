use crate::colours::*;


// lmao I'm just having fun at this point

macro_rules! out
{
	() => {
		out!("")
	};

	// with implicit prefix
	(
		$msg:literal $(, $($args:tt)* )?
	) => {
		out!(&format!($msg $(, $($args)* )?))
	};

	(
		$msg:expr
	) => {
		_out_(
			&format!("{GREY}│"),
			$msg
		)
	};

	// with prefix
	(
		$prefix:literal $(, $($args1:tt)* )?
		=> $msg:literal $(, $($args2:tt)* )?
	) => {
		_out_(
			&format!($prefix $(, $($args1)* )? ),
			&format!($msg $(, $($args2)* )?)
		)
	};

	(
		$prefix:literal $(, $($args:tt)* )? => $msg:expr
	) => {
		_out_(
			&format!($prefix $(, $($args)* )? ),
			$msg
		)
	};
}
pub(crate) use out;


macro_rules! ask
{
	($($args:tt)*) => {
		out!();
		out!("{Y}▸" => $($args)*);
	}
}
pub(crate) use ask;


macro_rules! head
{
	($msg:literal) => {
		out!();
		out!("├" => "{Y}{} {GREY}─────────────────", $msg);
	}
}
pub(crate) use head;


pub(crate) fn _out_(prefix: &str, message: &str)
{
	if message.is_empty() {
		println!(" {prefix}");
	} else {
		println!(" {prefix}  {W}{message}");
	}
}
