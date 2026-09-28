use crate::*;

use assertables::*;


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

/// Squarkdown preserves basic Markdown syntax.
#[test] fn markdown()
{
	clear_files("render/markdown").unwrap();
	assert!( squarkup_from("render/markdown").success() );

	let main = read_file("render/markdown/main/+page.svx");

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

	let main = read_file("render/strip-charm/main/+page.svx");
	assert_not_contains!( main, "#SQUARK" );
}
