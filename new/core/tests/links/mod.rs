use crate::*;


/// Squarkdown rewrites links.
#[test] fn basic()
{
	assert!( squarkup_from("tests/links/basic").success() );
}

/// Squarkdown rewrites links to folders outside a file's parent folder.
#[test] fn nested()
{
	assert!( squarkup_from("tests/links/nested").success() );
}

/// Squarkdown resolves links with anchors (`path/to/page.md#anchor`) while keeping the anchor.
#[test] fn anchors()
{
	assert!( squarkup_from("tests/links/anchors").success() );
}

/// With `linked-file-inactive: error`, Squarkdown cesrash when encountering links to inactive pages.
#[test] fn inactive_crash()
{
	assert!( !squarkup_from("tests/links/inactive-crash").success() );
}
