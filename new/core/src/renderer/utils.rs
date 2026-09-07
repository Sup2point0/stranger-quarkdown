use super::*;

use crate::{
	PageData,
	utils::testing::*,
	utils::macros::*,
};

use std::io::Cursor;


pub(super) fn test_exact(cases: &[&str])
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
