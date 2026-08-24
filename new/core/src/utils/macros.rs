#[macro_export]
macro_rules! str {
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
