use crate::*;

use assertables::*;


/// Squarkdown errors on 2-page conflicts with `config.errors.strict = true`.
#[test] fn conflicts()
{
	let r = capture_squarkup_from("errors/conflicts");
	assert_contains!( r, "conflict" );
	assert_contains!( r, "left.md" );
	assert_contains!( r, "right.md" );
}

/// Squarkdown errors on 3-page conflicts with `config.errors.strict = true`.
#[test] fn many_conflicts()
{
	let r = capture_squarkup_from("errors/many-conflicts");
	assert_contains!( r, "conflict" );
	assert_contains!( r, "1.md" );
	assert_contains!( r, "2.md" );
	assert_contains!( r, "3.md" );
}
