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
		let output = renderer.render_from(source, &PageData::default(), &TEST_CONFIG).unwrap();

		assert_eq!( &output, source );
	}
}

/// Run the renderer over `cases`, checking that the rendered output is exactly identical to the input, with `config.format.preserve_comments` enabled.
pub(super) fn test_preserves_with_comments(cases: &[&str])
{
	let mut config = TEST_CONFIG.clone();
	config.format.preserve_comments = true;

	for source in cases {
		let mut renderer = Renderer::new();
		let output = renderer.render_from(source, &PageData::default(), &config).unwrap();

		assert_eq!( &output, source );
	}
}

/// Run the renderer over `cases`, checking that each input renders to its expected output.
pub(super) fn test_expected(cases: &[(&str, &str)])
{
	for (source, expected) in cases {
		let mut renderer = Renderer::new();
		let output = renderer.render_from(source, &PageData::default(), &TEST_CONFIG).unwrap();

		assert_eq!( &output, expected );
	}
}
