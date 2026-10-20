// lmao I'm just having fun at this point

macro_rules! out
{
	() => {
		_out_("│", "")
	};

	(
		$msg:expr
	) => {
		out(
			&format!("{GREY}│"),
			$msg
		)
	};

	(
		$msg:literal $(, $($args:tt)* )?
	) => {
		out!("{GREY}│", $msg:literal $(, $($args:tt)* )?)
	};

	(
		$prefix:literal $(, $($args:tt)* )? => $msg:expr
	) => {
		_out_(
			&format!($prefix $(, $($args)* )? ),
			$msg
		)
	};

	(
		$prefix:literal $(, $($args1:tt)* )? => $msg:literal $(, $($args2:tt)* )?
	) => {
		_out_(
			&format!($prefix $(, $($args1)* )? ),
			&format!($msg $(, $($args2)* )?)
		)
	};
}
pub(super) use out;


pub(super) fn _out_(prefix: &str, message: &str)
{
	if message.is_empty() {
		println!(" {prefix}");
	} else {
		println!(" {prefix}  {message}");
	}
}
