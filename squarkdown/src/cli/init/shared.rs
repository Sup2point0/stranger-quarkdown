use std::io;


/// Alias for an infallible `writeln!`, used for appending to the config `String`.
macro_rules! append
{
	(  $($args:tt)*  ) => {
		let _ = writeln!($($args)*);
	}
}
pub(crate) use append;


pub type AbandonOrResult<E> = Result<(), AbandonError<E>>;

pub enum AbandonError<E>
{
	ABANDON,
	Other(E),
}

impl From<io::Error> for AbandonError<io::Error>
{
	fn from(value: io::Error) -> Self {
		Self::Other(value)
	}
}
