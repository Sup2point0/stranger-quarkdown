use super::*;

use std::io::Cursor;


pub(super) fn test_exact(
	cases: &[&'static str],
	test: impl Fn(CharmParser<Cursor<&&str>>, &str),
)
{
	for case in cases {
		let cursor = Cursor::new(case);
		let parser = CharmParser::init(cursor, None).unwrap();

		test(parser, case)
	}
}

pub(super) fn test_expected<X>(
	cases: &[(&'static str, X)],
	test: impl Fn(CharmParser<Cursor<&&str>>, &X),
)
{
	for (source, expected) in cases {
		let cursor = Cursor::new(source);
		let parser = CharmParser::init(cursor, None).unwrap();

		test(parser, expected)
	}
}
