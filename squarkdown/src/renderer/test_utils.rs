use super::*;
use crate::config::*;
use crate::macros::*;
use crate::utils::testing::*;

use assertables::*;
use pretty_assertions::{ assert_eq };


/// Run the renderer over `cases`, checking that the rendered output is exactly identical to the input.
pub(super) fn test_preserves(cases: &[&str])
{
	test_preserves_for(|_| {}, cases);
}

/// Run the renderer over `cases`, checking that the rendered output is exactly identical to the input.
pub(super) fn test_preserves_for(
	change_config: impl FnOnce(&mut SquarkupConfig),
	cases: &[&str],
)
{
	let pairs: Vec<(&str, &str)> = cases.iter().map(|case| (*case, *case)).collect();
	test_expected_for(change_config, &pairs);
}

/// Run the renderer over `cases`, checking that each input renders to `expected`.
pub(super) fn test_expect(cases: &[&str], expected: &str)
{
	test_expect_for(|_| {}, cases, expected);
}

/// Apply `change_config`, then run the renderer over `cases`, checking that each input renders to its expected output.
pub(super) fn test_expect_for(
	change_config: impl FnOnce(&mut SquarkupConfig),
	cases: &[&str],
	expected: &str,
)
{
	let mut config = TEST_CONFIG.clone();
	change_config(&mut config);

	for source in cases {
		let mut renderer = Renderer::new(&TEST_PAGE, &TEST_SITE, &config);
		let output = renderer.render_from(source.trim());
		assert_eq!( output.trim(), expected.trim(), "{:?}", renderer.ctx.printed() );
	}
}

/// Run the renderer over `cases`, checking that each input renders to its expected output.
pub(super) fn test_expected(cases: &[(&str, &str)])
{
	test_expected_for(|c| c.errors.on_error = ErrorAction::KILL, cases)
}

/// Apply `change_config`, then run the renderer over `cases`, checking that each input renders to its expected output.
pub(super) fn test_expected_for(
	change_config: impl FnOnce(&mut SquarkupConfig),
	cases: &[(&str, &str)],
)
{
	let mut config = TEST_CONFIG.clone();
	change_config(&mut config);

	for (source, expected) in cases {
		let mut renderer = Renderer::new(&TEST_PAGE, &TEST_SITE, &config);
		let output = renderer.render_from(source.trim());
		assert!( renderer.errors.is_fine(), "{Y}{:?}{R}", renderer.errors );
		assert_is_empty!( renderer.ctx );
		assert_eq!( output.trim(), expected.trim(), "{Y}{:?}{R}", renderer.ctx.printed() );
	}
}
