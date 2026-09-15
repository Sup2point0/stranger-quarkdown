use crate::*;

use assertables::*;


/// Squarkdown rewrites internal links.
#[test] fn basic()
{
	assert!( squarkup_from("links/basic").success() );

	let main = read_file("links/basic/main/+page.svx");
	let side = read_file("links/basic/side/+page.svx");
	assert_contains!( main, "[side](side)" );
	assert_contains!( side, "[main](main)" );
	assert_not_contains!( main, "[side](./side)" );
	assert_not_contains!( main, "[main](./main)" );
}

/// Squarkdown rewrites links to folders outside a file's parent folder.
#[test] fn nested()
{
	assert!( squarkup_from("links/nested").success() );
}

/// Squarkdown does not resolve external links, and rewrites links with `<sup>↗</sup>` to `<a target="_blank">`.
#[test] fn external()
{
	assert!( squarkup_from("links/external").success() );
}

/// Squarkdown resolves links with anchors (`path/to/page.md#anchor`) while keeping the anchor.
#[test] fn anchors()
{
	assert!( squarkup_from("links/anchors").success() );
}

/// Squarkdown crashes when encountering links to nonexistent files.
#[test] fn broken_crash()
{
	assert!( !squarkup_from("links/broken").success() );
}

/// With `linked-file-inactive: error`, Squarkdown crashes when encountering links to inactive pages.
#[test] fn inactive_crash()
{
	assert!( !squarkup_from("links/inactive-crash").success() );
}
