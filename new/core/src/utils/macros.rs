#[macro_export]
macro_rules! str {
	()        => { String::new() };
	($t:expr) => { String::from($t) };
} pub use str;

#[macro_export]
macro_rules! bx {
	()        => { Box::new() };
	($t:expr) => { Box::new($t) };
} pub use bx;

#[macro_export]
macro_rules! fmt {
	($($args:tt)*) => { format!($($args)*) };
} pub use fmt;


#[macro_export]
macro_rules! strings
{
	() => {
		tiny_vec!([String; 4])
	};
	($($values:expr),* $(,)?) => {
		tiny_vec!( [String; 4] => $(str!($values)),* )
	};
}
pub use strings;


#[macro_export]
macro_rules! dir
{
	($base:literal $(/ $part:expr)*) => { $base$(.join($part))* };
	($base:ident $($field:ident).* $(/ $part:expr)*) => { $base$(.$field)*$(.join($part))* };
}
pub use dir;


#[macro_export]
macro_rules! err
{
	() => { |e| $crate::errors::SquarkError::external(e) }
}
pub use err;


/// Format a string with a single path argument, normalising `\` in the path to `/`.
#[macro_export]
macro_rules! slash
{
	($msg:literal, $path:expr) => {
		if let Some(normalised) = path_slash::PathBufExt::to_slash(&$path) {
			fmt!($msg, normalised)
		} else {
			fmt!($msg, $path.display())
		}
	};
}
pub use slash;


#[macro_export]
macro_rules! all
{
	($obj:expr =>
		$( . $method:ident ( $($args:expr),* $(,)? ) ),*
		$(,)?
	) =>
	{
		{
			let obj = $obj;
			$( obj.$method( $($args),* ) )&&*
		}
	}
}
pub use all;
