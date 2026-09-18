use crate::*;

use assertables::*;


/// Squarkdown exports files to the right place.
#[test] fn root_only()
{
	clear_files("sources/root-only").unwrap();
	assert!( squarkup_from("sources/root_only").success() );

	let main = read_file("sources/root-only/main/+page.svx");
	assert_contains!( main, "squarkup" );
	assert!( !TEST_SITE.join("sources/root-only/ignore").exists() );
}
