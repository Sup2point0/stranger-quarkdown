use crate::*;

use assertables::*;
use path_macro::path;

use std::fs;


/// Squarkdown tracks series and picks up an index page explicitly declared with `| index =`.
#[test] fn explicit_index()
{
	let path = path!(*TEST_SITE / "src/data/series/explicit-index.json");

	if path.exists() {
		fs::remove_file(&path).unwrap();
	}

	assert!( squarkup_from("series/explicit-index").success() );
	assert!( path.is_file() );

	let data = read_file("../data/series/explicit-index.json");

	assert_contains!( data, "series\": [\"suppety\"]" );

	assert_contains!( data, "suppety\": {\n" );
	assert_contains!( data, "index_page\": \"index.md\"" );
	assert_contains!( data, "pages\": [\"readme.md\"]" );
	
	assert_contains!( data, "tags\": {}" );
}

/// Squarkdown tracks series and picks up an index page implicitly declared with `| series =` instead of `| index =`.
#[test] fn implicit_index()
{
	let path = path!(*TEST_SITE / "src/data/series/explicit-index.json");

	if path.exists() {
		fs::remove_file(&path).unwrap();
	}

	assert!( squarkup_from("series/implicit-index").success() );
	assert!( path.is_file() );

	let data = read_file("../data/series/implicit-index.json");

	assert_contains!( data, "series\": [\"soup\"]" );

	assert_contains!( data, "soup\": {\n" );
	assert_contains!( data, "index_page\": \"index.md\"" );
	assert_contains!( data, "pages\": [\"readme.md\"]" );
	
	assert_contains!( data, "tags\": {}" );
}
