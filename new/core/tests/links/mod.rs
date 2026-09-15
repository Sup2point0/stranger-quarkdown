use crate::*;


/// Squarkdown rewrites internal links.
#[test] fn basic()
{
	assert!( squarkup_from("tests/links/basic").success() );
}

/// Squarkdown rewrites links to folders outside a file's parent folder.
#[test] fn nested()
{
	assert!( squarkup_from("tests/links/nested").success() );
}

/// Squarkdown does not resolve external links, and rewrites links with `<sup>↗</sup>` to `<a target="_blank">`.
#[test] fn external()
{
	assert!( squarkup_from("tests/links/external").success() );
}

/// Squarkdown resolves links with anchors (`path/to/page.md#anchor`) while keeping the anchor.
#[test] fn anchors()
{
	assert!( squarkup_from("tests/links/anchors").success() );
}

/// Squarkdown crashes when encountering links to nonexistent files.
#[test] fn broken_crash()
{
	assert!( !squarkup_from("tests/links/broken").success() );
}

/// With `linked-file-inactive: error`, Squarkdown crashes when encountering links to inactive pages.
#[test] fn inactive_crash()
{
	assert!( !squarkup_from("tests/links/inactive-crash").success() );
}
