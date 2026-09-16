use crate::*;

use assertables::*;


/// Squarkdown errors on page conflicts with `config.errors.strict = true`.
#[test] fn conflicts()
{
	assert!( !squarkup_from("errors/conflicts").success() );
}
