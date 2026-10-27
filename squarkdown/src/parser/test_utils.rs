use super::*;
use crate::utils::testing::*;


pub(super) fn test_exact(
	cases: &[&'static str],
	test: impl Fn(CharmParser, &str),
)
{
	test_exact_for(|_| {}, cases, test)
}

pub(super) fn test_exact_for(
	change_config: impl FnOnce(&mut SquarkupConfig),
	cases: &[&'static str],
	test: impl Fn(CharmParser, &str),
)
{
	let mut config = TEST_CONFIG.clone();
	change_config(&mut config);

	for case in cases {
		let parser = CharmParser::new(case, TEST_FILE.clone(), &config);

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
