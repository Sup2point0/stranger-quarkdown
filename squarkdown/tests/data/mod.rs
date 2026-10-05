use crate::*;

use assertables::*;
use path_macro::path;

use std::fs;


/// Squarkdown saves site data to JSON.
#[test] fn basic()
{
	let path = path!(*TEST_SITE / "src/data/basic.json");

	if path.exists() {
		fs::remove_file(&path).unwrap();
	}

	assert!( squarkup_from("data/basic").success() );
	assert!( path.is_file() );

	let data = read_file("../data/basic.json");

	assert_contains!( data, "stats\": {" );
	assert_contains!( data, "built_on\": \"" );
	assert_contains!( data, "checked_files\": 2" );
	assert_contains!( data, "active_pages\": 1" );
	assert_contains!( data, "assets\": 0" );
	
	assert_contains!( data, "pages\": {" );
	assert_contains!( data, "main.md\": {" );
	assert_contains!( data, "filepath\": \"main.md\"," );
	assert_contains!( data, "destination\": \"main\"," );
	assert_contains!( data, "flags\": []," );
	assert_contains!( data, "title\": \"Charm Squark\"," );
	assert_contains!( data, "description\": \"Just testing charm squarks are handled properly in production\"," );
	assert_contains!( data, "heading\": \"Charm Squark\"," );
	assert_contains!( data, "caption\": \"Testing testing\"," );
	assert_contains!( data, "tags\": [\n" );
	assert_contains!( data, "release_date\": \"2020-03-31\"," );
	assert_contains!( data, "release_date_raw\": \"2020 March 31\"," );
	assert_contains!( data, "last_update\": \"2020-04-01\"," );
	assert_contains!( data, "last_update_raw\": \"2020 April 1\"," );
	assert_contains!( data, "other\": {}" );

	assert_contains!( data, "work?\": [\n" );
	assert_contains!( data, "does\": [\n" );
	assert_contains!( data, "this\": [\n" );
	assert_contains!( data, "all\": [\n" );
}
