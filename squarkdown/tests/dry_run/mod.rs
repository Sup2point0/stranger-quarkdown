use crate::*;
use squarkdown::utils;

use assertables::*;
use path_macro::path;

use std::fs::{ self, File };


/// Squarkdown does not produce side effects when `--dry-run` is enabled.
#[test] fn dry_run()
{
	let gitkeep = path!(*TEST_ROUTES / "dry-run" / ".gitkeep");

	let (status, out) = capture_squarkup!("dry_run", "--dry-run");
	assert!( status.success() );
	assert_contains!( out, "2/2" );

	let _ = fs::remove_file(&gitkeep);
	assert!( utils::is_empty_dir(path!(*TEST_ROUTES / "dry-run")).unwrap() );
	assert_not!( path!(*TEST_SITE / "src" / "dry-run.json").exists() );
	assert_not!( path!(*TEST_SITE / "static" / "nothing.svg").exists() );

	File::create(&gitkeep).unwrap();
}
