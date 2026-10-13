use crate::*;

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
