use crate::*;

use assertables::*;


/// Squarkdown exports files to the right place.
#[test] fn dest()
{
	clear_files("render/dest").unwrap();
	assert!( squarkup_from("render/dest").success() );

	let f1 = read_file("render/dest/1/+page.svx");
	let f2 = read_file("render/dest/nest/2/+page.svx");
	let f3 = read_file("render/dest/nest/nest/3/+page.svx");
	let f4 = read_file("render/dest/nest/nest/nest/4/+page.svx");
	let f5 = read_file("render/dest/nest/nest/nest/nest/5/+page.svx");
	assert_contains!( f1, "One" );
	assert_contains!( f2, "Two" );
	assert_contains!( f3, "Three" );
	assert_contains!( f4, "Four" );
	assert_contains!( f5, "Five" );
}
