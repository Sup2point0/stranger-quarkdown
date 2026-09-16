use crate::*;

use assertables::*;


/// Squarkdown errors on page conflicts with `config.errors.strict = true`.
#[test] fn conflicts()
{
	let r = capture_squarkup_from("errors/conflicts");
	assert_contains!( r, "conflict" );
}
