use crate::*;

use path_macro::path;

use std::fs;


/// Squarkdown copies assets to `static/`.
#[test] fn basic()
{
	let path = path!(*TEST_SITE / "static" / "basic.svg");

	if path.exists() {
		fs::remove_file(&path).unwrap();
	}

	assert!( squarkup!("assets/basic", "--assets").success() );
	assert!( path.is_file() )
}

/// Squarkdown copies site assets directly to the root of `static/`.
#[test] fn site()
{
	let path = path!(*TEST_SITE / "static" / "site.svg");

	if path.exists() {
		fs::remove_file(&path).unwrap();
	}

	assert!( squarkup!("assets/site", "--assets").success() );
	assert!( path.is_file() )
}
