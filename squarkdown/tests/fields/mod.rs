use crate::*;

use assertables::*;


/// Squarkdown renders `+page.ts` and outputs `site.json` with long field names.
#[test] fn long()
{
	clear_files("fields/long").unwrap();

	assert!( squarkup_from("fields/long").success() );

	let ts = read_file("fields/long/+page.ts");
	assert_contains!( ts, "stranger-quarkdown" );
	assert_contains!( ts, "PageData<\"long\">" );
	assert_contains!( ts, "filepath: \"readme.md\"," );
	assert_contains!( ts, "destination: \"\"" );
	assert_contains!( ts, "flags: [\"feat\", \"dev\"]" );
	assert_contains!( ts, "title: \"Long\"" );
	assert_contains!( ts, "description: \"This page" );
	assert_contains!( ts, "heading: \"Long Field Names\"" );
	assert_contains!( ts, "tags: [\"1\", \"2\", \"3\"]" );
	assert_contains!( ts, "release_date: new Date(2020, 0, 1)" );
	assert_contains!( ts, "release_date_raw: \"2020\"" );
	assert_contains!( ts, "last_update: new Date(2020, 3, 1)" );
	assert_contains!( ts, "last_update_raw: \"2020 April\"" );

	let json = read_file("../data/long.json");
	assert_contains!( json, "filepath\": \"readme.md\"" );
	assert_contains!( json, "destination\": \"\"" );
	assert_contains!( json, "flags\": [\n" );
	assert_contains!( json, "\"feat\"," );
	assert_contains!( json, "\"dev\"" );
	assert_contains!( json, "title\": \"Long\"" );
	assert_contains!( json, "description\": \"This page" );
	assert_contains!( json, "heading\": \"Long Field Names\"" );
	assert_contains!( json, "caption\": \"Longer\"," );
	assert_contains!( json, "tags\": [" );
	assert_contains!( json, "\"1\"," );
	assert_contains!( json, "\"2\"," );
	assert_contains!( json, "\"3\"" );
	assert_contains!( json, "release_date\": \"2020-01-01\"," );
	assert_contains!( json, "release_date_raw\": \"2020\"" );
	assert_contains!( json, "last_update\": \"2020-04-01\"," );
	assert_contains!( json, "last_update_raw\": \"2020 April\"" );
}

/// Squarkdown renders `+page.ts` and outputs `site.json` with short field names.
#[test] fn short()
{
	clear_files("fields/short").unwrap();

	assert!( squarkup_from("fields/short").success() );

	let ts = read_file("fields/short/+page.ts");
	assert_contains!( ts, "stranger-quarkdown" );
	assert_contains!( ts, "PageData<\"short\">" );
	assert_contains!( ts, "path: \"readme.md\"," );
	assert_contains!( ts, "dest: \"\"" );
	assert_contains!( ts, "flags: [\"feat\", \"dev\"]" );
	assert_contains!( ts, "title: \"Short\"" );
	assert_contains!( ts, "desc: \"This page" );
	assert_contains!( ts, "head: \"Short Field Names\"" );
	assert_contains!( ts, "tags: [\"1\", \"2\", \"3\"]" );
	assert_contains!( ts, "date: new Date(2020, 0, 1)" );
	assert_contains!( ts, "date_raw: \"2020\"" );
	assert_contains!( ts, "update: new Date(2020, 3, 1)" );
	assert_contains!( ts, "update_raw: \"2020 April\"" );

	let json = read_file("../data/short.json");
	assert_contains!( json, "path\": \"readme.md\"" );
	assert_contains!( json, "dest\": \"\"" );
	assert_contains!( json, "flags\": [\n" );
	assert_contains!( json, "\"feat\"," );
	assert_contains!( json, "\"dev\"" );
	assert_contains!( json, "title\": \"Short\"" );
	assert_contains!( json, "desc\": \"This page" );
	assert_contains!( json, "head\": \"Short Field Names\"" );
	assert_contains!( json, "capt\": \"Shorter\"," );
	assert_contains!( json, "tags\": [" );
	assert_contains!( json, "\"1\"," );
	assert_contains!( json, "\"2\"," );
	assert_contains!( json, "\"3\"" );
	assert_contains!( json, "date\": \"2020-01-01\"," );
	assert_contains!( json, "date_raw\": \"2020\"" );
	assert_contains!( json, "update\": \"2020-04-01\"," );
	assert_contains!( json, "update_raw\": \"2020 April\"" );
}

/// Squarkdown renders `+page.ts` and outputs `site.json` with arbitrary fields.
#[test] fn arbitrary()
{
	clear_files("fields/arbitrary").unwrap();

	assert!( squarkup_from("fields/arbitrary").success() );
	let ts = read_file("fields/arbitrary/+page.ts");
	assert_contains!( ts, "PageData<\"long\">" );
	assert_contains!( ts, "path: \"readme.md\"," );
	assert_contains!( ts, "title: \"Arbitrary\"," );
	assert_contains!( ts, "description: \"This page" );
	assert_contains!( ts, "caption: \"The following fields" );
	assert_contains!( ts, "\"one\": [\"1\"]," );
	assert_contains!( ts, "\"two\": [\"2\"]," );
	assert_contains!( ts, "\"three\": [\"3\"]," );
	assert_contains!( ts, "\"some_list\": [\"yes\", \"no\"]," );
	assert_contains!( ts, "\"some_array\": [\"true\", \"false\"]," );
	assert_contains!( ts, "\"dangerous.key\": [\"val1\", \"val2\", \"val3\"]," );
}
