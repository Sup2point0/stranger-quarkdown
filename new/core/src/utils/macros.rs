#[macro_export]
macro_rules! str {
	()        => { String::new() };
	($t:expr) => { String::from($t) };
}
pub use str;


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
	() => { |e| $crate::errors::SquarkError::External(Box::new(e)) }
}
pub use err;


/// Format a string with a single path argument, normalising `\` in the path to `/`.
#[macro_export]
macro_rules! slash
{
	($msg:literal, $path:expr) => {
		if let Some(normalised) = path_slash::PathBufExt::to_slash(&$path) {
			format!($msg, normalised)
		} else {
			format!($msg, $path.display())
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
