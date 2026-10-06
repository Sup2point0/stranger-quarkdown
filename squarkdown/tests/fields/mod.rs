use crate::*;

use assertables::*;


/// Squarkdown renders `+page.ts` with long field names.
#[test] fn long()
{
	clear_files("fields/long").unwrap();

	assert!( squarkup_from("fields/long").success() );

	let main = read_file("fields/long/+page.ts");
	assert_contains!( main, "PageData<\"long\">" );
	assert_contains!( main, "filepath: \"readme.md\"" );
	assert_contains!( main, "destination: \"\"" );
	assert_contains!( main, "flags: [\"feat\", \"dev\"]" );
	assert_contains!( main, "title: \"Long\"" );
	assert_contains!( main, "description: \"This page" );
	assert_contains!( main, "heading: \"Long Field Names\"" );
	assert_contains!( main, "tags: [\"1\", \"2\", \"3\"]" );
	assert_contains!( main, "release_date: new Date(2020, 0, 1)" );
	assert_contains!( main, "release_date_raw: \"2020\"" );
	assert_contains!( main, "last_update: new Date(2020, 3, 1)" );
	assert_contains!( main, "last_update_raw: \"2020 April\"" );

	let data = read_file("../data/long.json");
	assert_contains!( data, "filepath\": \"readme.md\"" );
	assert_contains!( data, "destination\": \"\"" );
	assert_contains!( data, "flags\": [\n" );
	assert_contains!( data, "\"feat\"," );
	assert_contains!( data, "\"dev\"" );
	assert_contains!( data, "title\": \"Long\"" );
	assert_contains!( data, "description\": \"This page" );
	assert_contains!( data, "heading\": \"Long Field Names\"" );
	assert_contains!( data, "caption\": \"Longer\"," );
	assert_contains!( data, "tags\": [" );
	assert_contains!( data, "\"1\"," );
	assert_contains!( data, "\"2\"," );
	assert_contains!( data, "\"3\"" );
	assert_contains!( data, "release_date\": \"2020-01-01\"," );
	assert_contains!( data, "release_date_raw\": \"2020\"" );
	assert_contains!( data, "last_update\": \"2020-04-01\"," );
	assert_contains!( data, "last_update_raw\": \"2020 April\"" );
}

/// Squarkdown renders `+page.ts` with short field names.
#[test] fn short()
{
	clear_files("fields/short").unwrap();

	assert!( squarkup_from("fields/short").success() );

	let main = read_file("fields/short/+page.ts");
	assert_contains!( main, "PageData<\"short\">" );
	assert_contains!( main, "path: \"readme.md\"" );
	assert_contains!( main, "dest: \"\"" );
	assert_contains!( main, "flags: [\"feat\", \"dev\"]" );
	assert_contains!( main, "title: \"Short\"" );
	assert_contains!( main, "desc: \"This page" );
	assert_contains!( main, "head: \"Short Field Names\"" );
	assert_contains!( main, "tags: [\"1\", \"2\", \"3\"]" );
	assert_contains!( main, "date: new Date(2020, 0, 1)" );
	assert_contains!( main, "date_raw: \"2020\"" );
	assert_contains!( main, "update: new Date(2020, 3, 1)" );
	assert_contains!( main, "update_raw: \"2020 April\"" );

	let data = read_file("../data/short.json");
	assert_contains!( data, "path\": \"readme.md\"" );
	assert_contains!( data, "dest\": \"\"" );
	assert_contains!( data, "flags\": [\n" );
	assert_contains!( data, "\"feat\"," );
	assert_contains!( data, "\"dev\"" );
	assert_contains!( data, "title\": \"Short\"" );
	assert_contains!( data, "desc\": \"This page" );
	assert_contains!( data, "head\": \"Short Field Names\"" );
	assert_contains!( data, "capt\": \"Shorter\"," );
	assert_contains!( data, "tags\": [" );
	assert_contains!( data, "\"1\"," );
	assert_contains!( data, "\"2\"," );
	assert_contains!( data, "\"3\"" );
	assert_contains!( data, "date\": \"2020-01-01\"," );
	assert_contains!( data, "date_raw\": \"2020\"" );
	assert_contains!( data, "update\": \"2020-04-01\"," );
	assert_contains!( data, "update_raw\": \"2020 April\"" );
}
