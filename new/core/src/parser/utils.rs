use std::io::Cursor;

use super::*;
use crate::utils::*;


#[macro_export]
macro_rules! err_msg
{
	() => {
		|| String::from("INTERNAL INVARIANT BROKEN")
	};

	($msg:expr $(, $args:expr)* $(,)?) => {
		|| format!($msg, $($args)*)
	}
}

pub(super) use err_msg;


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
