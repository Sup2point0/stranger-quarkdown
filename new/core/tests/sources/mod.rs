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

/// Squarkdown respects `paths.sources`.
#[test] fn fixed()
{
	clear_files("sources/fixed").unwrap();
	assert!( squarkup_from("sources/fixed").success() );

	let main = read_file("sources/fixed/main/+page.svx");
	assert_contains!( main, "squarkup" );
	assert!( !TEST_SITE.join("sources/fixed/ignore").exists() );
}

/// Squarkdown respects `paths.include` patterns.
#[test] fn include()
{
	clear_files("sources/include").unwrap();
	assert!( squarkup_from("sources/include").success() );

	let main = read_file("sources/include/main/+page.svx");
	assert_contains!( main, "squarkup" );
	assert!( !TEST_SITE.join("sources/include/ignore").exists() );
}

/// Squarkdown respects `paths.exclude` patterns.
#[test] fn exclude()
{
	clear_files("sources/exclude").unwrap();
	assert!( squarkup_from("sources/exclude").success() );

	let main = read_file("sources/exclude/main/+page.svx");
	let side = read_file("sources/exclude/nested/side/+page.svx");
	assert_contains!( main, "squarkup" );
	assert_contains!( side, "squarkup" );

	assert!( !TEST_SITE.join("sources/exclude/ignore").exists() );
	assert!( !TEST_SITE.join("sources/exclude/nested/ignore").exists() );
	assert!( !TEST_SITE.join("sources/exclude/nested/bad1").exists() );
	assert!( !TEST_SITE.join("sources/exclude/nested/bad2").exists() );
	assert!( !TEST_SITE.join("sources/exclude/nested/bad3").exists() );
	
	let main = read_file("sources/exclude/nested/bad/but/keep/+page.svx");
	assert_contains!( main, "squarkup" );
}
