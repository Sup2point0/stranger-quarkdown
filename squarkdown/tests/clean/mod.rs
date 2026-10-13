use crate::*;

use assertables::*;
use path_macro::path;


/// Squarkdown cleans up output correctly.
#[test] fn basic()
{
	assert!( squarkup_from("clean/basic").success() );
	assert!( path!(*TEST_ROUTES / "clean/basic/main/+page.svx").exists() );
	assert!( path!(*TEST_ROUTES / "clean/basic/main/+page.ts").exists() );

	assert!( squarkup!("clean/basic", "clean").success() );
	assert_not!( path!(*TEST_ROUTES / "clean/basic/main/+page.svx").exists() );
	assert_not!( path!(*TEST_ROUTES / "clean/basic/main/+page.ts").exists() );
	assert_not!( path!(*TEST_ROUTES / "clean/basic/main").exists() );
	assert!( path!(*TEST_ROUTES / "clean/basic").exists() );
}

/// Squarkdown does not delete tracked files.
#[test] fn committed()
{
	assert!( squarkup_from("clean/committed").success() );
	assert!( path!(*TEST_ROUTES / "clean/committed/main/+page.svx").exists() );
	assert!( path!(*TEST_ROUTES / "clean/committed/main/+page.ts").exists() );

	let md = read_file("clean/committed/main/+page.svx");
	assert_contains!( md, "handwritten" );

	assert_not!( squarkup!("clean/committed", "clean").success() );
	assert_not!( path!(*TEST_ROUTES / "clean/committed/main/+page.ts").exists() );
	assert!( path!(*TEST_ROUTES / "clean/committed/main/+page.svx").exists() );
}
