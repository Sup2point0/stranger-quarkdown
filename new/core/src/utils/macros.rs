// == STRINGS == //

#[macro_export]
macro_rules! str {
	()        => { String::new() };
	($t:expr) => { String::from($t) };
} pub use str;

#[macro_export]
macro_rules! fmt {
	($($args:tt)*) => { format!($($args)*) };
} pub use fmt;

/// Format a string with a single path argument, normalising `\` in the path to `/`.
#[macro_export]
macro_rules! slash {
	($msg:literal, $path:expr) => {
		{
			let path: &std::path::Path = &$path;

			match path_slash::PathExt::to_slash(path) {
				Some(p) => format!($msg, p),
				None    => format!($msg, path.display()),
			}
		}
	};
	($msg:literal, $path:expr, $($args:tt)*) => {
		{
			let path: &std::path::Path = &$path;

			match path_slash::PathExt::to_slash(path) {
				Some(p) => format!($msg, p, $($args)*),
				None    => format!($msg, path.display(), $($args)*),
			}
		}
	};
} pub use slash;

/// Lazily produce a string for an error path.
#[macro_export]
macro_rules! to {
	()             => { || String::from("INTERNAL INVARIANT HAS BEEN BROKEN") };
	($($args:tt)*) => { || format!($($args)*) };
} pub use to;

/// Lazily produce a string for an error path.
#[macro_export]
macro_rules! hints {
	()             => { || String::from("INTERNAL INVARIANT HAS BEEN BROKEN") };
	($($args:tt)*) => { || format!($($args)*) };
} pub use hints;


// == STRUCTS == //

#[macro_export]
macro_rules! pair {
	($t:expr) => { ($t, $t) }
} pub use pair;

#[macro_export]
macro_rules! bx {
	()        => { Box::new() };
	($t:expr) => { Box::new($t) };
} pub use bx;

/// Join paths using a `/` separator.
/// 
/// ```ignore
/// dir!(base / rel / file.rs)
/// ```
#[macro_export]
macro_rules! dir {
	($base:literal $(/ $part:expr)*) => { $base$(.join($part))* };
	($base:ident $(.$field:ident)* $(/ $part:expr)*) => { $base$(.$field)*$(.join($part))* };
} pub use dir;

#[macro_export]
macro_rules! strings {
	() => {
		tiny_vec!([String; 4])
	};
	($($values:expr),* $(,)?) => {
		tiny_vec!( [String; 4] => $(str!($values)),* )
	};
} pub use strings;


// == ERRORS == //

/// Lazily produce a `SquarkError::External`.
#[macro_export]
macro_rules! err {
	() => { |e| $crate::errors::SquarkError::external(e) }
} pub use err;


/// Scope `?` try fallbacks to a local scope, instead of the entire containing function.
#[macro_export]
macro_rules! catch
{
	($errs:ident => $eval:block) => {
		if let Err(e) = (|| -> SquarkResult<_> { $eval; Ok(()) })() {
			$errs.push(e);
		}
	};
	($($body:tt)*) => {
		(|| -> SquarkResult<_> { $($body)*; Ok(()) })()
	};
} pub use catch;

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
} pub use all;
