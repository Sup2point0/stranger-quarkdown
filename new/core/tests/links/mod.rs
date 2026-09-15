use std::assert_matches;


/// Squarkdown rewrites links.
#[test] fn basic()
{
	let bin = env!("CARGO_BIN_EXE_squarkdown");

	let r = std::process::Command::new(bin)
		.current_dir(std::env::current_dir().unwrap().join("tests/links/basic"))
		.status();

	assert_matches!( r, Ok(..) );
	assert!( r.unwrap().success() );
}

/// Squarkdown resolves links with anchors (`path/to/page.md#anchor`) while keeping the anchor.
#[test] fn anchors()
{
	let bin = env!("CARGO_BIN_EXE_squarkdown");

	let r = std::process::Command::new(bin)
		.current_dir(std::env::current_dir().unwrap().join("tests/links/anchors"))
		.status();

	assert_matches!( r, Ok(..) );
	assert!( r.unwrap().success() );
}

/// With `linked-file-inactive: error`, Squarkdown cesrash when encountering links to inactive pages.
#[test] fn inactive()
{
	let bin = env!("CARGO_BIN_EXE_squarkdown");

	let r = std::process::Command::new(bin)
		.current_dir(std::env::current_dir().unwrap().join("tests/links/inactive-crash"))
		.status();

	assert_matches!( r, Ok(..) );
	assert!( !r.unwrap().success() );
}
