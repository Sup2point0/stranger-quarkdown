use crate::*;

use assertables::*;
use path_macro::path;


/// Squarkdown ignores files with `#SQUARK dead!`.
#[test] fn dead()
{
	let (status, out) = capture_squarkup_from("render/dead");
	assert_not!( status.success() );
	assert_contains!( out, "no active files found" );

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
	assert_contains!( base, "Infer" );
	assert_contains!( main, "Main" );
	assert_contains!( nest, "Infer" );
	assert_contains!( side, "Side" );
}

/// Squarkdown preserves basic Markdown syntax.
#[test] fn markdown()
{
	clear_files("render/markdown").unwrap();
	assert!( squarkup_from("render/markdown").success() );

	let main = read_file("render/markdown/+page.svx");

	assert_contains!( main, "\n## Level 2\n" );
	assert_contains!( main, "\n### Level 3\n" );
	assert_contains!( main, "\n#### Level 4\n" );

	assert_contains!( main, " *italic* " );
	assert_contains!( main, " **bold** " );
	assert_contains!( main, " ~~strikethrough~~ " );
	assert_contains!( main, " ***italic bold*** " );
	assert_contains!( main, " *~~italic strikethrough~~* " );
	assert_contains!( main, " **~~bold strikethrough~~**." );

	assert_contains!( main, "\n- item 1\n" );
	assert_contains!( main, "\n- item 2\n" );
	assert_contains!( main, "\n- item 3\n" );

	assert_contains!( main, "\n  - item 1.1\n" );
	assert_contains!( main, "\n  - item 1.2\n" );
	assert_contains!( main, "\n    - item 1.2.1\n" );
	assert_contains!( main, "\n    - item 1.2.2\n" );
	assert_contains!( main, "\n    - item 1.2.3\n" );
	assert_contains!( main, "\n  - item 1.3\n" );

	assert_contains!( main, "\n1. one\n" );
	assert_contains!( main, "\n1. two\n" );
	assert_contains!( main, "\n1. three\n" );

	assert_contains!( main, "\nimplicit  \nbreak\n" );
	assert_contains!( main, "\nexplicit   \nbreak\n" );

	assert_contains!( main, "\n---\n" );

	assert_contains!( main, "\n > \n > Quote\n" );
	assert_contains!( main, "\n > \n > Multi\n > Line\n > \n > Quote" );
}

/// Squarkdown strips the charm squark even when `config.format.preserve-comments` is enabled.
#[test] fn strip_charm()
{
	clear_files("render/strip-charm").unwrap();
	assert!( squarkup_from("render/strip-charm").success() );

	let main = read_file("render/strip-charm/+page.svx");
	assert_not_contains!( main, "#SQUARK" );
}

/// Squarkdown strips the charm squark even when `config.format.preserve-comments` is enabled.
#[test] fn no_page_ts()
{
	clear_files("render/no-page-ts").unwrap();
	assert!( squarkup_from("render/no-page-ts").success() );

	assert!( path!(*TEST_ROUTES / "render/no-page-ts/+page.svx").exists() );
	assert_not!( path!(*TEST_ROUTES / "render/no-page-ts/+page.ts").exists() );
}
