/// Alias for an infallible `writeln!`, used for appending to the config `String`.
macro_rules! append
{
	(  $($args:tt)*  ) => {
		let _ = writeln!($($args)*);
	}
}
pub(crate) use append;
