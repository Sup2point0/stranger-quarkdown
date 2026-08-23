#[cfg(test)] use std::io::Cursor;

#[cfg(test)] use super::*;
#[cfg(test)] use crate::utils::*;


/// Lazily produce an error message for the parser's error path.
#[macro_export]
macro_rules! when {
	() => { || String::from("INTERNAL INVARIANT HAS BEEN BROKEN") };

	($msg:expr $(, $args:expr)* $(,)?) => {
		|| format!($msg, $($args)*)
	}
}

#[macro_export]
macro_rules! to {
	() => { || String::from("INTERNAL INVARIANT HAS BEEN BROKEN") };

	($msg:expr $(, $args:expr)* $(,)?) => {
		|| format!($msg, $($args)*)
	}
}

pub(super) use when;
pub(super) use to;


#[cfg(test)]
pub(super) fn test_exact(cases: &[&'static str], test: impl Fn(CharmParser<Cursor<&&str>>, &str))
{
	for case in cases {
		let cursor = Cursor::new(case);
		let parser = CharmParser::init(cursor, &TEST_CONFIG).unwrap();

		test(parser, case)
	}
}

#[cfg(test)]
pub(super) fn test_expected<X>(
	cases: &[(&'static str, X)],
	test: impl Fn(CharmParser<Cursor<&&str>>, &X),
)
{
	for (source, expected) in cases {
		let cursor = Cursor::new(source);
		let parser = CharmParser::init(cursor, &TEST_CONFIG).unwrap();

		test(parser, expected)
	}
}
