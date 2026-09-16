use crate::*;

use assertables::*;


/// Squarkdown rewrites internal links.
#[test] fn basic()
{
	clear_files("links/basic").unwrap();
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
	clear_files("links/nested").unwrap();
	assert!( squarkup_from("links/nested").success() );

	let main = read_file("links/nested/main/+page.svx");
	let side = read_file("links/nested/nested/side/+page.svx");
	let more = read_file("links/nested/more/+page.svx");

	assert_contains!( main, "[side](nested/side)" );
	assert_contains!( side, "[main](../main)" );
	assert_not_contains!( main, "[side](./nested/side)" );

	assert_contains!( main, "[more](more)" );
	assert_contains!( side, "[more](../more)" );
	assert_contains!( more, "[side](nested/side)" );
	assert_contains!( more, "[main](main)" );
}

/// Squarkdown does not resolve external links, and rewrites links with `<sup>↗</sup>` to `<a target="_blank">`.
#[test] fn external()
{
	clear_files("links/external").unwrap();
	assert!( squarkup_from("links/external").success() );

	let main = read_file("links/external/main/+page.svx");
	assert_contains!( main, "[GitHub](https://github.com/Sup2point0/stranger-quarkdown)" );
	// assert_contains!( main, "<a target=\"_blank\" href=\"https://svelte.dev\">Svelte</a>" );
}

/// Squarkdown resolves links with anchors (`path/to/page.md#anchor`) while keeping the anchor.
#[test] fn anchors()
{
	clear_files("links/anchors").unwrap();
	assert!( squarkup_from("links/anchors").success() );
	
	let main = read_file("links/anchors/main/+page.svx");
	let side = read_file("links/anchors/side/+page.svx");
	assert_contains!( main, "[side](side#section)" );
	assert_contains!( side, "[main](main)" );
}

/// Squarkdown crashes when encountering links to nonexistent files.
#[test] fn broken_crash()
{
	// TODO check specific error
	clear_files("links/broken").unwrap();
	assert!( !squarkup_from("links/broken").success() );
}

/// With `linked-file-inactive: error`, Squarkdown crashes when encountering links to inactive pages.
#[test] fn inactive_crash()
{
	// TODO check specific error
	clear_files("links/inactive-crash").unwrap();
	assert!( !squarkup_from("links/inactive-crash").success() );
}
