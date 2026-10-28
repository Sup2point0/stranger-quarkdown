use crate::*;

use assertables::*;
use path_macro::path;


/// Squarkdown cleans up output correctly.
#[test] fn basic()
{
	assert!( squarkup_from("clean/basic").success() );
	assert!( path!(*TEST_ROUTES / "clean/basic/main/+page.svx").exists() );
	assert!( path!(*TEST_ROUTES / "clean/basic/main/+page.ts").exists() );

	assert!( squarkup!("clean/basic", "clean", "--dry-run").success() );
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

	assert_not!( squarkup!("clean/committed", "clean", "--dry-run").success() );
	assert!( path!(*TEST_ROUTES / "clean/committed/main/+page.svx").exists() );
	assert!( path!(*TEST_ROUTES / "clean/committed/main/+page.ts").exists() );
	let md = read_file("clean/committed/main/+page.svx");
	assert_contains!( md, "handwritten" );

	assert_not!( squarkup!("clean/committed", "clean").success() );
	assert_not!( path!(*TEST_ROUTES / "clean/committed/main/+page.ts").exists() );
	assert!( path!(*TEST_ROUTES / "clean/committed/main/+page.svx").exists() );
}


/// Squarkdown only deletes `+page.svx` and `+page.ts` files, leaving the folder intact if other files are inside.
#[test] fn keep_folder()
{
	assert!( squarkup_from("clean/keep-folder").success() );
	assert!( path!(*TEST_ROUTES / "clean/keep-folder/+page.svx").exists() );
	assert!( path!(*TEST_ROUTES / "clean/keep-folder/+page.ts").exists() );

	assert!( squarkup!("clean/keep-folder", "clean", "--dry-run").success() );
	assert!( path!(*TEST_ROUTES / "clean/keep-folder/+page.svx").exists() );
	assert!( path!(*TEST_ROUTES / "clean/keep-folder/+page.ts").exists() );

	assert!( squarkup!("clean/keep-folder", "clean").success() );
	assert_not!( path!(*TEST_ROUTES / "clean/keep-folder/+page.ts").exists() );
	assert_not!( path!(*TEST_ROUTES / "clean/keep-folder/+page.svx").exists() );
	assert!( path!(*TEST_ROUTES / "clean/keep-folder/.gitignore").exists() );
}
