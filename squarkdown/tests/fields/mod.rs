use crate::*;

use assertables::*;


/// Squarkdown renders `+page.ts` with long field names.
#[test] fn long()
{
	clear_files("fields/long").unwrap();

	assert!( squarkup_from("fields/long").success() );

	let main = read_file("fields/long/+page.ts");
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
	assert_contains!( main, "other: {}" );
}

/// Squarkdown renders `+page.ts` with short field names.
#[test] fn short()
{
	clear_files("fields/short").unwrap();

	assert!( squarkup_from("fields/short").success() );

	let main = read_file("fields/short/+page.ts");
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
	assert_contains!( main, "other: {}" );
}
