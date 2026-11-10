use crate::*;

use assertables::*;
use path_macro::path;


/// Squarkdown ignores files with `#SQUARK dead!`.
#[test] fn dead()
{
	let (status, out) = capture_squarkup_from("render/dead");
	assert_not!( status.success() );
	assert_contains!( out, "No active files found" );

	assert_not!( path!(*TEST_ROUTES / "render/dead/only-dead" ).exists() );
	assert_not!( path!(*TEST_ROUTES / "render/dead/live-and-dead" ).exists() );
	assert_not!( path!(*TEST_ROUTES / "render/dead/dead-and-live" ).exists() );
}

/// Squarkdown exports files to the right place.
#[test] fn dest()
{
	clear_files("render/dest").unwrap();
	assert!( squarkup_from("render/dest").success() );

	let f1 = read_file("render/dest/1/+page.svx");
	let f2 = read_file("render/dest/nest/2/+page.svx");
	let f3 = read_file("render/dest/nest/nest/3/+page.svx");
	let f4 = read_file("render/dest/nest/nest/nest/4/+page.svx");
	let f5 = read_file("render/dest/nest/nest/nest/nest/5/+page.svx");
	assert_contains!( f1, "One" );
	assert_contains!( f2, "Two" );
	assert_contains!( f3, "Three" );
	assert_contains!( f4, "Four" );
	assert_contains!( f5, "Five" );
}

/// Squarkdown infers the destination when none is explicitly provided.
#[test] fn dest_infer()
{
	clear_files("render/dest-infer").unwrap();
	assert!( squarkup_from("render/dest-infer").success() );

	let base = read_file("render/dest-infer/+page.svx");
	let main = read_file("render/dest-infer/main/+page.svx");
	let nest = read_file("render/dest-infer/nested/+page.svx");
	let side = read_file("render/dest-infer/nested/side/+page.svx");
	assert_contains!( base, "test-project/src/routes/render/dest-infer/" );
	assert_contains!( main, "test-project/src/routes/render/dest-infer/main/" );
	assert_contains!( nest, "test-project/src/routes/render/dest-infer/nested/" );
	assert_contains!( side, "test-project/src/routes/render/dest-infer/nested/side/" );
}

/// Squarkdown preserves basic Markdown syntax.
#[test] fn markdown()
{
	clear_files("render/markdown").unwrap();
	assert!( squarkup_from("render/markdown").success() );

	let md = read_file("render/markdown/+page.svx");

	assert_contains!( md, "\n## Level 2\n" );
	assert_contains!( md, "\n### Level 3\n" );
	assert_contains!( md, "\n#### Level 4\n" );

	assert_contains!( md, " *italic* " );
	assert_contains!( md, " **bold** " );
	assert_contains!( md, " ~~strikethrough~~ " );
	assert_contains!( md, " ***italic bold*** " );
	assert_contains!( md, " *~~italic strikethrough~~* " );
	assert_contains!( md, " **~~bold strikethrough~~**." );

	assert_contains!( md, "\n- item 1\n" );
	assert_contains!( md, "\n- item 2\n" );
	assert_contains!( md, "\n- item 3\n" );

	assert_contains!( md, "\n  - item 1.1\n" );
	assert_contains!( md, "\n  - item 1.2\n" );
	assert_contains!( md, "\n    - item 1.2.1\n" );
	assert_contains!( md, "\n    - item 1.2.2\n" );
	assert_contains!( md, "\n    - item 1.2.3\n" );
	assert_contains!( md, "\n  - item 1.3\n" );

	assert_contains!( md, "\n1. one\n" );
	assert_contains!( md, "\n1. two\n" );
	assert_contains!( md, "\n1. three\n" );

	assert_contains!( md, "\nimplicit  \nbreak\n" );
	assert_contains!( md, "\nexplicit   \nbreak\n" );

	assert_contains!( md, "\n---\n" );

	assert_contains!( md, "\n > \n > Quote\n" );
	assert_contains!( md, "\n > \n > Multi\n > Line\n > \n > Quote" );
}

/// Squarkdown preserves basic HTML syntax.
#[test] fn html()
{
	clear_files("render/html").unwrap();
	assert!( squarkup_from("render/html").success() );

	let md = read_file("render/html/+page.svx");
	assert_contains!( md, "\nProse\n" );
	assert_contains!( md, "\nAnd <strong>more</strong> prose!" );

	assert_contains!( md, "\n<h2> Level 2 </h2>\n" );
	assert_contains!( md, "\n<h3> Level 3 </h3>\n" );
	assert_contains!( md, "\n<h4> Level 4 </h4>\n" );

	assert_contains!( md, " <em>italic</em> " );
	assert_contains!( md, " <strong>bold</strong> " );
	assert_contains!( md, " <strong><em>italic bold</em></strong>." );

	assert_contains!( md, "\n<ul>\n" );
	assert_contains!( md, "\n\t<li> item 1 </li>\n" );
	assert_contains!( md, "\n\t<li> item 2 </li>\n" );
	assert_contains!( md, "\n\t<li> item 3 </li>\n" );
	assert_contains!( md, "\n</ul>\n" );

	assert_contains!( md, "\n<table>\n" );
	assert_contains!( md, "\n<tbody><tr> \t<td> sup </td>\n" );
	assert_contains!( md, "\n</tr></tbody></table>\n" );
}

/// Squarkdown strips the charm squark even when `config.format.preserve-comments` is enabled.
#[test] fn strip_charm()
{
	clear_files("render/strip-charm").unwrap();
	assert!( squarkup_from("render/strip-charm").success() );

	let main = read_file("render/strip-charm/+page.svx");
	assert_not_contains!( main, "#SQUARK" );
}

/// Squarkdown injects `<svelte:head>` with `<title>` and `<meta name="description">`, escaping quotes properly.
#[test] fn head()
{
	clear_files("render/head").unwrap();
	assert!( squarkup_from("render/head").success() );

	let md = read_file("render/head/+page.svx");
	assert_contains!( md, "<svelte:head>\n" );
	assert_contains!( md, "<title> Sup, World! · Squarkdown Tests </title>\n" );
	assert_contains!( md, "<meta name=\"description\" content=\"The quick brown fox &quot;jumps&quot; over the lazy dog\" />\n" );
	assert_contains!( md, "</svelte:head>\n" );
}

/// Squarkdown strips the charm squark even when `config.format.preserve-comments` is enabled.
#[test] fn no_page_ts()
{
	clear_files("render/no-page-ts").unwrap();
	assert!( squarkup_from("render/no-page-ts").success() );

	assert!( path!(*TEST_ROUTES / "render/no-page-ts/+page.svx").exists() );
	assert_not!( path!(*TEST_ROUTES / "render/no-page-ts/+page.ts").exists() );
}
