macro_rules! str {
	($t:expr) => {
		String::from($t)
	};
}
pub(crate) use str;


macro_rules! strings
{
	() => {
		tiny_vec!([String; 4])
	};
	($value:expr $(, $rest:expr)* $(,)?) => {
		tiny_vec!([String; 4] => str!($value) $(, str!($rest))*)
	};
}
pub(crate) use strings;


macro_rules! dir {
	($base:ident $(/ $part:expr)*) => {
		$base$(.join($part))*
	};
}
pub(crate) use dir;
