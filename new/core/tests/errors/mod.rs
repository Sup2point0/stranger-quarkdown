use crate::*;

use assertables::*;


/// Squarkdown exits if it finds no files to squarkup.
#[test] fn none()
{
	let (status, out) = capture_squarkup_from("errors/none");
	assert!( !status.success() );
	assert_contains!( out, "no files found to squarkup" );
}

/// Squarkdown errors on 2-page conflicts with `config.errors.strict = true`.
#[test] fn conflicts_crashes()
{
	let (status, out) = capture_squarkup_from("errors/conflicts");
	assert!( !status.success() );
	assert_contains!( out, "conflicting" );
	assert_contains!( out, "left.md" );
	assert_contains!( out, "right.md" );
	assert!( !TESTS.join("errors/conflicts/top").exists() );
}

/// Squarkdown errors on 3-page conflicts with `config.errors.strict = true`.
#[test] fn many_conflicts_crashes()
{
	let (status, out) = capture_squarkup_from("errors/many-conflicts");
	assert!( !status.success() );
	assert_contains!( out, "conflicting" );
	assert_contains!( out, "1.md" );
	assert_contains!( out, "2.md" );
	assert_contains!( out, "3.md" );
	assert!( !TESTS.join("errors/many-conflicts/nested/top").exists() );
}

#[test] fn file_already_exists_crashes()
{
	let (status, out) = capture_squarkup_from("errors/already");
	assert!( !status.success() );
	assert_contains!( out, "cannot overwrite" );

	let svx = read_file("errors/already/main/+page.svx");
	let ts  = read_file("errors/already/main/+page.ts");
	assert_contains!( svx, "shouldn't be overwrriten" );
	assert_contains!( ts,  "Don't overwrite me" );
}
