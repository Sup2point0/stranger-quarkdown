// == STRINGS == //

#[macro_export]
macro_rules! str {
	()        => { String::new() };
	($t:expr) => { String::from($t) };
} pub use str;

/// An alias for `format!()` that doesn’t require a string literal.
#[macro_export]
macro_rules! fmt {
	($msg:literal)  => { format!($msg) };
	($display:expr) => { format!("{}", $display) };
	($($args:tt)*)  => { format!($($args)*) };
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
	($($args:tt)*) => { || fmt!($($args)*) };
} pub use to;


// == STRUCTS == //

#[cfg(test)]
macro_rules! pair {
	($t:expr) => { ($t, $t) }
}
#[cfg(test)]
pub(crate) use pair;

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
		if let Err(e) = (|| -> SquarkResult<_> {
			$body;
			Ok(())
		})() {
			$errs.push(e);
		}
	};
	($($body:tt)*) => {
		(|| -> SquarkResult<_> {
			$($body)*;
			Ok(())
		})()
	};
} pub use catch;


// == ARITHMETIC == //

macro_rules! zero
{
	($left:ident - $right:literal) => {
		$left.saturating_sub($right)
	};
	($left:ident - $right:ident) => {
		$left.saturating_sub($right)
	};
	($var:ident . $attr:ident - $right:literal) => {
		$var.$attr.saturating_sub($right)
	};
	($var:ident . $attr:ident - $right:ident) => {
		$var.$attr.saturating_sub($right)
	};
}
pub(crate) use zero;
