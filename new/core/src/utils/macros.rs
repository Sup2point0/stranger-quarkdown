#[macro_export]
macro_rules! str {
	($t:expr) => {
		String::from($t)
	};
}
pub use str;


#[macro_export]
macro_rules! strings
{
	() => {
		tiny_vec!([String; 4])
	};
	($value:expr $(, $rest:expr)* $(,)?) => {
		tiny_vec!([String; 4] => str!($value) $(, str!($rest))*)
	};
}
pub use strings;


#[macro_export]
macro_rules! dir
{
	($base:literal $(/ $part:expr)*) => { $base$(.join($part))* };
	($base:ident $(. $field:ident)* $(/ $part:expr)*) => { $base$(.$field)*$(.join($part))* };
}
pub use dir;
