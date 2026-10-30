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
	assert_contains!( main, "[empty link]()" );
	assert_not_contains!( main, "[side](./side)" );
	assert_not_contains!( main, "[main](./main)" );
}

/// Squarkdown rewrites links to files in folders outside a file's parent folder.
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

/// Squarkdown rewrites links to folders without a specific file.
#[test] fn folders()
{
	clear_files("links/folders").unwrap();
	assert!( squarkup_from("links/folders").success() );
	
	let main = read_file("links/folders/main/+page.svx");
	let side = read_file("links/folders/nested/+page.svx");
	let root = read_file("links/folders/root/+page.svx");
	assert_contains!( main, "[nested](nested)" );
	assert_contains!( main, "[nested/](nested)" );
	assert_contains!( main, "[nested/more](root)" );
	assert_contains!( main, "[nested/more/](root)" );
	assert_contains!( side, "[main](main)" );
	assert_contains!( root, "[main](main)" );
}

/// Squarkdown does not resolve external links, and rewrites links with `<sup>↗</sup>` to `<a target="_blank">`.
#[test] fn external()
{
	clear_files("links/external").unwrap();
	assert!( squarkup_from("links/external").success() );

	let main = read_file("links/external/+page.svx");
	assert_contains!( main, "[GitHub](https://github.com/Sup2point0/stranger-quarkdown)" );
	assert_contains!( main, "<a target=\"_blank\" href=\"https://svelte.dev\">Svelte</a>" );
}

/// Squarkdown rewrites links with anchors (`path/to/page.md#anchor`) while keeping the anchor.
#[test] fn anchors()
{
	clear_files("links/anchors").unwrap();
	assert!( squarkup_from("links/anchors").success() );
	
	let main = read_file("links/anchors/main/+page.svx");
	let side = read_file("links/anchors/side/+page.svx");
	assert_contains!( main, "[later](#later)" );
	assert_contains!( main, "[much later](#much-later)" );
	assert_contains!( main, "[side](side#section)" );
	assert_contains!( side, "[main](main)" );
}

/// Squarkdown rewrites links with titles (`path/to/page.md 'title'`) while keeping the title.
#[test] fn titles()
{
	clear_files("links/titles").unwrap();
	assert!( squarkup_from("links/titles").success() );
	
	let main = read_file("links/titles/+page.svx");
	assert_contains!( main, "[self](titles \"self\")" );
	assert_contains!( main, "[GitHub](https://github.com/Sup2point0/stranger-quarkdown \"squarkdown\")" );
}

/// Squarkdown normalises links with `%20`-encoded spaces, rewriting `%20` to `-`.
#[test] fn spaces()
{
	clear_files("links/spaces").unwrap();
	assert!( squarkup_from("links/spaces").success() );
	
	let main = read_file("links/spaces/the-main-file/+page.svx");
	let side = read_file("links/spaces/other-file/+page.svx");
	assert_contains!( main, "[other file](other-file)" );
	assert_contains!( main, "[Section in the other file](other-file#section)" );
	assert_contains!( side, "[the main file](the-main-file)" );
	assert_contains!( side, "[Section in the main file](the-main-file#section)" );
}

/// Squarkdown crashes when encountering links to nonexistent files.
#[test] fn broken_crashes()
{
	clear_files("links/broken-crash").unwrap();

	let (status, out) = capture_squarkup_from("links/broken-crash");
	assert_not!( status.success() );
	assert_contains!( out, "broken link" );
	assert_contains!( out, "(side.md)" );
	assert_contains!( out, "(./side.md)" );
	assert_contains!( out, "(nested/side.md)" );
	assert_contains!( out, "(./nested/side.md)" );
}

/// With `link-broken = mark-dead`, Squarkdown rewrites links to inactive pages into HTML links with a `dead-link` class.
#[test] fn broken_mark()
{
	clear_files("links/broken-mark").unwrap();
	assert!( squarkup_from("links/broken-mark").success() );

	let md = read_file("links/broken-mark/+page.svx");
	assert_contains!( md, "<a" );
	assert_contains!( md, "<a class=\"dead-link\" href=\"?\">side</a>" );
	assert_contains!( md, "<a class=\"dead-link\" href=\"?\">./side</a>" );
	assert_contains!( md, "<a class=\"dead-link\" href=\"?\">nested</a>" );
	assert_contains!( md, "<a class=\"dead-link\" href=\"?\">nested/</a>" );
	assert_contains!( md, "<a class=\"dead-link\" href=\"?\">nested/side</a>" );
	assert_contains!( md, "<a class=\"dead-link\" href=\"?\">./nested/side</a>" );
}

/// With `link-inactive = error`, Squarkdown crashes when encountering links to inactive pages.
#[test] fn inactive_crashes()
{
	clear_files("links/inactive-crash").unwrap();

	let (status, out) = capture_squarkup_from("links/inactive-crash");
	assert_not!( status.success() );
	assert_contains!( out, "inactive page" );
	assert_contains!( out, "(./inactive.md)" );
}

/// With `link-inactive = mark-dead`, Squarkdown rewrites links to inactive pages into HTML links with a `dead-link` class.
#[test] fn inactive_mark()
{
	clear_files("links/inactive-mark").unwrap();
	assert!( squarkup_from("links/inactive-mark").success() );

	let md = read_file("links/inactive-mark/+page.svx");
	assert_contains!( md, "<a" );
	assert_contains!( md, "<a class=\"dead-link\" href=\"?\">inactive</a>" );
	assert_contains!( md, "<a class=\"dead-link\" href=\"?\">very inactive</a>" );
}

/// With `link-inactive = strip-extension`, Squarkdown rewrites links to inactive pages into links to the original file on GitHub.
#[test] fn strip_extension()
{
	clear_files("links/strip-extension").unwrap();
	assert!( squarkup_from("links/strip-extension").success() );

	let md = read_file("links/strip-extension/+page.svx");
	assert_contains!( md, "[side](side)" );
}

/// With `link-inactive = link-to-github`, Squarkdown rewrites links to inactive pages into links to the original file on GitHub.
#[test] fn link_to_github()
{
	clear_files("links/link-github").unwrap();
	assert!( squarkup_from("links/link-github").success() );

	let md = read_file("links/link-github/+page.svx");
	assert_contains!( md, "[github](https://github.com/Sup2point0/stranger-quarkdown/blob/main/readme.md)" );
}


/// Squarkdown rewrites links to asset files.
#[test] fn assets()
{
	clear_files("links/assets").unwrap();
	assert!( squarkup!("links/assets", "--assets").success() );
	
	let main = read_file("links/assets/+page.svx");
	assert_contains!( main, "![one](/link.jpg)" );
	assert_contains!( main, "![two](/link.jpg)" );
	assert_contains!( main, "![three](/nested/linked.jpg)" );
	assert_contains!( main, "![four](/nested/linked.jpg)" );
	assert_not_contains!( main, ".assets" );
	assert_contains!( main, "![&lbrack;one&rbrack;](/link.jpg)" );
	assert_contains!( main, "![&lbrack;&lbrack;one&rbrack;&rbrack;](/link.jpg)" );
}

/// Squarkdown rewrites links to site asset files.
#[test] fn site_assets()
{
	clear_files("links/site-assets").unwrap();
	assert!( squarkup!("links/site-assets", "--assets").success() );
	
	let md = read_file("links/site-assets/+page.svx");
	assert_contains!( md, "![one](/link.jpg)" );
	assert_contains!( md, "![two](/link.jpg)" );
	assert_contains!( md, "![three](/linked.jpg)" );
	assert_contains!( md, "![four](/linked.jpg)" );
	assert_contains!( md, "![five](/nested/linking.jpg)" );
	assert_contains!( md, "![six](/nested/linking.jpg)" );
}

/// Squarkdown rewrites links to asset files _and_ other files together.
#[test] fn assets_linked()
{
	clear_files("links/assets-linked").unwrap();
	assert!( squarkup!("links/assets-linked", "--assets").success() );
	
	let main = read_file("links/assets-linked/main/+page.svx");
	assert_contains!( main, "[![one](/link.jpg)](side)" );
	assert_contains!( main, "[![two](/link.jpg)](side)" );
	assert_contains!( main, "[![three](/link.jpg)](side)" );
	assert_contains!( main, "[![four](/link.jpg)](side)" );
	assert_not_contains!( main, ".assets" );
	assert_not_contains!( main, ".md" );
}
