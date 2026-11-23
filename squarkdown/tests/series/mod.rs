use crate::*;

use assertables::*;
use path_macro::path;

use std::fs;


/// Squarkdown tracks series and index pages for them.
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
	assert_contains!( data, "index-page\": \"index.md\"" );
	assert_contains!( data, "pages\": [\"readme.md\"]" );
	
	assert_contains!( data, "tags\": {}" );
}
