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
	( $msg:literal, $path:expr $(, $($args:tt)*)? ) => {
		{
			let path: &std::path::Path = &$path;

			match path_slash::PathExt::to_slash(path) {
				Some(p) => format!($msg, p, $($($args)*)?),
				None    => format!($msg, path.display(), $($($args)*)?),
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


// == STRUCTS == //

#[allow(unused)]
macro_rules! pair {
	($t:expr) => { ($t, $t) }
} pub(crate) use pair;

macro_rules! strings {
	() => {
		tiny_vec!([String; 1])
	};
	($($values:expr),* $(,)?) => {
		tiny_vec!( [String; 1] => $(String::from($values)),* )
	};
} pub(crate) use strings;


// == ERRORS == //

/// Scope `?` try fallbacks to a local scope, instead of the entire containing function.
/// 
/// Automatically returns `Ok(())` at the end.
#[macro_export]
macro_rules! catch
{
	($errs:expr => $body:block) => {
		if let Err(e) = (|| -> SquarkResult<_> { $body; Ok(()) })() {
			$errs.push(e);
		}
	};
	($($body:tt)*) => {
		(|| -> SquarkResult<_> { $($body)*; Ok(()) })()
	};
} pub use catch;
