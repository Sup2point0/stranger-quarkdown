use super::*;

use crate::{
	PageData,
	macros::*,
	utils::testing::*,
};

use std::io::Cursor;


/// Run the renderer over `cases`, checking that the rendered output is exactly identical to the input.
pub(super) fn test_preserves(cases: &[&str])
{
	for case in cases {
		let source = Cursor::new(case);
		let target = Cursor::new(vec![]);

		let mut renderer = Renderer::init(source, target, TESTS.clone(), TESTS.clone()).unwrap();
		renderer.render(&PageData::default(), &TEST_CONFIG).unwrap();

		let output = String::from_utf8(renderer._writer.get_ref().clone().into_inner()).unwrap();
		assert_eq!( output, str!(*case) );
	}
}

/// Run the renderer over `cases`, checking that the rendered output is exactly identical to the input, with `config.format.preserve_comments` enabled.
pub(super) fn test_preserves_with_comments(cases: &[&str])
{
	let mut config = TEST_CONFIG.clone();
	config.format.preserve_comments = true;

	for case in cases {
		let source = Cursor::new(case);
		let target = Cursor::new(vec![]);

		let mut renderer = Renderer::init(source, target, TESTS.clone(), TESTS.clone()).unwrap();
		renderer.render(&PageData::default(), &config).unwrap();

		let output = String::from_utf8(renderer._writer.get_ref().clone().into_inner()).unwrap();
		assert_eq!( output, str!(*case) );
	}
}

/// Run the renderer over `cases`, checking that each input renders to its expected output.
pub(super) fn test_expected(cases: &[(&str, &str)])
{
	for (case, expected) in cases {
		let source = Cursor::new(case);
		let target = Cursor::new(vec![]);

		let mut renderer = Renderer::init(source, target, TESTS.clone(), TESTS.clone()).unwrap();
		renderer.render(&PageData::default(), &TEST_CONFIG).unwrap();

		let output = String::from_utf8(renderer._writer.get_ref().clone().into_inner()).unwrap();
		assert_eq!( output, str!(*expected) );
	}
}
