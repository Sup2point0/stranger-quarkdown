use crate::*;

use assertables::*;
use path_macro::path;

use std::fs;


/// Squarkdown saves site data to JSON.
#[test] fn site_data()
{
	let path = path!(*TEST_SITE / "src/data/site-data.json");

	if path.exists() {
		fs::remove_file(&path).unwrap();
	}

	assert!( squarkup_from("fields/site-data").success() );
	assert!( path.is_file() );

	let data = read_file("../data/site-data.json");

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
	assert_contains!( data, "tags\": [\"does\", \"this\", \"all\", \"work?\"]," );
	assert_contains!( data, "release_date\": \"2020-03-31\"," );
	assert_contains!( data, "release_date_raw\": \"2020 March 31\"," );
	assert_contains!( data, "last_update\": \"2020-04-01\"," );
	assert_contains!( data, "last_update_raw\": \"2020 April 1\"" );

	assert_contains!( data, "work?\": [\n" );
	assert_contains!( data, "does\": [\n" );
	assert_contains!( data, "this\": [\n" );
	assert_contains!( data, "all\": [\n" );
}

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
	assert_contains!( json, "flags\": [\"feat\", \"dev\"]" );
	assert_contains!( json, "title\": \"Long\"" );
	assert_contains!( json, "description\": \"This page" );
	assert_contains!( json, "heading\": \"Long Field Names\"" );
	assert_contains!( json, "caption\": \"Longer\"," );
	assert_contains!( json, "tags\": [\"1\", \"2\", \"3\"]" );
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
	assert_contains!( ts, "path: \"readme.md\"" );
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
	assert_contains!( json, "flags\": [\"feat\", \"dev\"]" );
	assert_contains!( json, "title\": \"Short\"" );
	assert_contains!( json, "desc\": \"This page" );
	assert_contains!( json, "head\": \"Short Field Names\"" );
	assert_contains!( json, "capt\": \"Shorter\"" );
	assert_contains!( json, "tags\": [\"1\", \"2\", \"3\"]" );
	assert_contains!( json, "date\": \"2020-01-01\"" );
	assert_contains!( json, "date_raw\": \"2020\"" );
	assert_contains!( json, "update\": \"2020-04-01\"" );
	assert_contains!( json, "update_raw\": \"2020 April\"" );
}

/// Squarkdown renders `+page.ts` and outputs `site.json` with arbitrary fields.
#[test] fn arbitrary()
{
	clear_files("fields/arbitrary").unwrap();

	assert!( squarkup_from("fields/arbitrary").success() );

	let ts = read_file("fields/arbitrary/+page.ts");
	assert_contains!( ts, "PageData<\"long\">" );
	assert_contains!( ts, "PageData<\"long\"> & Record<OtherFields, string[]>" );
	assert_contains!( ts, "| \"one\"" );
	assert_contains!( ts, "| \"two\"" );
	assert_contains!( ts, "| \"three\"" );
	assert_contains!( ts, "| \"some_list\"" );
	assert_contains!( ts, "| \"some_array\"" );
	assert_contains!( ts, "| \"dangerous.key\"" );
	assert_contains!( ts, "| \"arbitrary\"" );
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

	let json = read_file("../data/arbitrary.json");
	assert_contains!( json, "\"one\": [\"1\"]" );
	assert_contains!( json, "\"two\": [\"2\"]" );
	assert_contains!( json, "\"three\": [\"3\"]" );
	assert_contains!( json, "\"some_list\": [\"yes\", \"no\"]" );
	assert_contains!( json, "\"some_array\": [\"true\", \"false\"]" );
	assert_contains!( json, "\"dangerous.key\": [\"val1\", \"val2\", \"val3\"]" );
	assert_contains!( json, "\"arbitrary\": [\"\\\"\\\"\"]" );
}

/// Squarkdown escapes values in `+page.ts` and `site.json`, so that characters like `"` don't break the output.
#[test] fn escape()
{
	clear_files("fields/escape").unwrap();

	assert!( squarkup_from("fields/escape").success() );
	let ts = read_file("fields/escape/+page.ts");
	assert_contains!( ts, "PageData<\"long\">" );
	assert_contains!( ts, "title: \"This title has \\\" in it\"," );
	assert_contains!( ts, "description: \"This description has \\\\ in it\"," );
	assert_contains!( ts, "caption: \"This caption has \\\\n in it\"," );
	assert_contains!( ts, "arbitrary\": [\"Maybe \\\"dangerous\\\"\"]," );
}
