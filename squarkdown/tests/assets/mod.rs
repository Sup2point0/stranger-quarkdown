use crate::*;

use path_macro::path;

use std::fs;


/// Squarkdown copies assets to `static/`.
#[test] fn basic()
{
	let path = path!(*TEST_SITE / "static/basic.svg");

	if path.exists() {
		fs::remove_file(&path).unwrap();
	}

	assert!( squarkup!("assets/basic", "--assets").success() );
	assert!( path.is_file() )
}

/// Squarkdown only copies assets with the relevant extensions.
#[test] fn extensions()
{
	clear_files("../../static/extensions").unwrap();

	assert!( squarkup!("assets/extensions", "--assets").success() );
	assert!( path!(*TEST_SITE / "static/extensions/ext.png").is_file() );
	assert!( path!(*TEST_SITE / "static/extensions/ext.jpg").is_file() );
	assert!( path!(*TEST_SITE / "static/extensions/ext.svg").is_file() );

	assert_not!( path!(*TEST_SITE / "static/extensions/ext.ttf").exists() );
	assert_not!( path!(*TEST_SITE / "static/extensions/ext.md").exists() );
	assert_not!( path!(*TEST_SITE / "static/extensions/ext").exists() );
}

/// Squarkdown copies site assets directly to the root of `static/`.
#[test] fn site()
{
	let path = path!(*TEST_SITE / "static/site.svg");

	if path.exists() {
		fs::remove_file(&path).unwrap();
	}

	assert!( squarkup!("assets/site", "--assets").success() );
	assert!( path.is_file() )
}
