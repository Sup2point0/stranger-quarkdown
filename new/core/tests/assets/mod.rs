use crate::*;

use assertables::*;
use path_macro::path;

use std::fs;


/// Squarkdown copies assets to `static/`.
#[test] fn basic()
{
	let path = path!(*TEST_SITE / "static" / "basic.svg");

	if path.exists() {
		fs::remove_file(&path).unwrap();
	}

	assert!( squarkup_from("assets/basic").success() );
	assert!( path.exists() )
}
