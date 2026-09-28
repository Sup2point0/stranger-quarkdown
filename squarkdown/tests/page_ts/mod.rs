use crate::*;

use assertables::*;


/// Squarkdown renders `+page.ts` with long field names.
#[test] fn long()
{
	clear_files("page-ts/long").unwrap();

	assert!( squarkup_from("page_ts/long").success() );

	let main = read_file("page-ts/long/main/+page.ts");
	assert_contains!( main, "filepath: \"main.md\"" );
	assert_contains!( main, "destination: \"main\"" );
	assert_contains!( main, "flags: []" );
	assert_contains!( main, "title: \"Long\"" );
	assert_contains!( main, "description: \"This page" );
	assert_contains!( main, "heading: \"Main\"" );
	assert_contains!( main, "tags: [\"1\", \"2\", \"3\"]" );
	assert_contains!( main, "release_date: new Date(2020, 1, 1)" );
	assert_contains!( main, "release_date_raw: \"2020\"" );
	assert_contains!( main, "last_update: new Date(2020, 3, 1)" );
	assert_contains!( main, "last_update_raw: \"2020 April\"" );
	assert_contains!( main, "other: {}" );
}

/// Squarkdown renders `+page.ts` with short field names.
#[test] fn short()
{
	clear_files("page-ts/short").unwrap();

	assert!( squarkup_from("page_ts/short").success() );

	let main = read_file("page-ts/short/main/+page.ts");
	assert_contains!( main, "path: \"main.md\"" );
	assert_contains!( main, "dest: \"main\"" );
	assert_contains!( main, "flags: []" );
	assert_contains!( main, "title: \"Short\"" );
	assert_contains!( main, "desc: \"This page" );
	assert_contains!( main, "head: \"Main\"" );
	assert_contains!( main, "tags: [\"1\", \"2\", \"3\"]" );
	assert_contains!( main, "date: new Date(2020, 1, 1)" );
	assert_contains!( main, "date_raw: \"2020\"" );
	assert_contains!( main, "update: new Date(2020, 4, 1)" );
	assert_contains!( main, "update_raw: \"2020 April\"" );
	assert_contains!( main, "other: {}" );
}
