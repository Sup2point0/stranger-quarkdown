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
#[test] fn conflicts()
{
	let (status, out) = capture_squarkup_from("errors/conflicts");
	assert!( !status.success() );
	assert_contains!( out, "conflict" );
	assert_contains!( out, "left.md" );
	assert_contains!( out, "right.md" );
}

/// Squarkdown errors on 3-page conflicts with `config.errors.strict = true`.
#[test] fn many_conflicts()
{
	let (status, out) = capture_squarkup_from("errors/many-conflicts");
	assert!( !status.success() );
	assert_contains!( out, "conflict" );
	assert_contains!( out, "1.md" );
	assert_contains!( out, "2.md" );
	assert_contains!( out, "3.md" );
}
