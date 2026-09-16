use super::*;
use crate::utils::testing::*;


pub(super) fn test_exact(
	cases: &[&'static str],
	test: impl Fn(CharmParser, &str),
)
{
	for case in cases {
		let parser = CharmParser::new(case, TEST_FILE.clone(), &TEST_CONFIG);

		test(parser, case)
	}
}

pub(super) fn test_expected<X>(
	cases: &[(&'static str, X)],
	test: impl Fn(CharmParser, &X),
)
{
	for (source, expected) in cases {
		let parser = CharmParser::new(source, TEST_FILE.clone(), &TEST_CONFIG);

		test(parser, expected)
	}
}
