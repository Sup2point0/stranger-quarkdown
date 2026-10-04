use crate::*;

use assertables::*;
use path_macro::path;

use std::fs;


/// Squarkdown injects fonts queries into `app.html`, both when they aren't present and when they're already present.
#[test] fn basic()
{
	let orig = read_file("../app.orig.html");
	let path = path!(*TEST_SITE / "src/app.html");
	fs::write(&path, &orig).unwrap();

	for _ in 0..2 {
		assert_not!( squarkup!("fonts", "--fonts").success() );
		assert!( path.is_file() );
		let new = read_file("../app.html");

		assert_contains!( new, "family=Montserrat" );
		assert_contains!( new, "family=Rajdhani" );
	}
}
