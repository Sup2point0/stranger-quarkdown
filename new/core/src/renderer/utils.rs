use super::*;

use crate::{
	PageData,
	macros::*,
	utils::testing::*,
};


/// Run the renderer over `cases`, checking that the rendered output is exactly identical to the input.
pub(super) fn test_preserves(cases: &[&str])
{
	for source in cases {
		let mut renderer = Renderer::new();
		let output = renderer.render_from(source.trim().to_owned(), &PageData::default(), &TEST_CONFIG);

		assert_eq!( output.trim(), source.trim(), "{:?}", renderer.ctx.stack() );
	}
}

/// Run the renderer over `cases`, checking that the rendered output is exactly identical to the input, with `config.format.preserve_comments` enabled.
pub(super) fn test_preserves_with_comments(cases: &[&str])
{
	let mut config = TEST_CONFIG.clone();
	config.format.preserve_comments = true;

	for source in cases {
		let mut renderer = Renderer::new();
		let output = renderer.render_from(source.trim().to_owned(), &PageData::default(), &config);

		assert_eq!( output.trim(), source.trim(), "{:?}", renderer.ctx.stack() );
	}
}

/// Run the renderer over `cases`, checking that each input renders to `expected`.
pub(super) fn test_expect(cases: &[&str], expected: &str)
{
	for source in cases {
		let mut renderer = Renderer::new();
		let output = renderer.render_from(source.trim().to_owned(), &PageData::default(), &TEST_CONFIG);

		assert_eq!( output.trim(), expected.trim(), "{:?}", renderer.ctx.stack() );
	}
}

/// Run the renderer over `cases`, checking that each input renders to its expected output.
pub(super) fn test_expected(cases: &[(&str, &str)])
{
	for (source, expected) in cases {
		let mut renderer = Renderer::new();
		let output = renderer.render_from(source.trim().to_owned(), &PageData::default(), &TEST_CONFIG);

		assert_eq!( output.trim(), expected.trim(), "{:?}", renderer.ctx.stack() );
	}
}
