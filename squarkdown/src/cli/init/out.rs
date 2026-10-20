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
pub(super) use out;


macro_rules! ask
{
	($($args:tt)*) => {
		out!("{Y}▸" => $($args)*)
	}
}
pub(super) use ask;


macro_rules! head
{
	($msg:literal) => {
		out!();
		out!("├" => "{Y}{}", $msg);
		out!();
	}
}
pub(super) use head;


pub(super) fn _out_(prefix: &str, message: &str)
{
	if message.is_empty() {
		println!(" {prefix}");
	} else {
		println!(" {prefix}  {W}{message}");
	}
}
