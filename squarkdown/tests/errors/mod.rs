use crate::*;

use assertables::*;
use path_macro::path;


#[test] fn invalid_config()
{
	let (status, out) = capture_squarkup_from("errors/invalid-config");
	assert_not!( status.success() );
	assert_contains!( out, "paths.sources" );
	assert_contains!( out, "paths.include" );
	assert_contains!( out, "paths.exclude" );
	assert_contains!( out, "out.folder" );
	assert_contains!( out, "out.file-name" );
	assert_contains!( out, "out.render-page-ts" );
	assert_contains!( out, "out.shorter-fields" );
	assert_contains!( out, "out.site-data-path" );
	assert_contains!( out, "format.preserve-heading" );
	assert_contains!( out, "format.preserve-comments" );
	assert_contains!( out, "format.externalise-links" );
	assert_contains!( out, "assets.folder" );
	assert_contains!( out, "assets.site-assets-folder" );

	assert_contains!( out, "must be a boolean" );
	assert_contains!( out, "must be a string" );
	assert_contains!( out, "must be an array" );

	assert_contains!( out, "(of folders relative to your project root)" );
	assert_contains!( out, "(of RegEx patterns)" );
	assert_contains!( out, "(folder relative to your SvelteKit site)" );
	assert_contains!( out, "(filename including" );
	assert_contains!( out, "(filepath including" );
	assert_contains!( out, "(folder relative to your project root)" );
}

/// Squarkdown exits if it finds no files to squarkup.
#[test] fn none()
{
	let (status, out) = capture_squarkup_from("errors/none");
	assert_not!( status.success() );
	assert_contains!( out, "no files found to squarkup" );
}

/// Squarkdown errors on 2-page conflicts with `config.errors.strict = true`.
#[test] fn conflicts_crashes()
{
	let (status, out) = capture_squarkup_from("errors/conflicts");
	assert_not!( status.success() );
	assert_contains!( out, "conflicting" );
	assert_contains!( out, "left.md" );
	assert_contains!( out, "right.md" );
	assert_not!( path!(*TESTS / "errors/conflicts/top").exists() );
}

/// Squarkdown errors on 3-page conflicts with `config.errors.strict = true`.
#[test] fn many_conflicts_crashes()
{
	let (status, out) = capture_squarkup_from("errors/many-conflicts");
	assert_not!( status.success() );
	assert_contains!( out, "conflicting" );
	assert_contains!( out, "1.md" );
	assert_contains!( out, "2.md" );
	assert_contains!( out, "3.md" );
	assert_not!( path!(*TESTS / "errors/many-conflicts/nested/top").exists() );
}

#[test] fn file_already_exists_crashes()
{
	let (status, out) = capture_squarkup_from("errors/already");
	assert_not!( status.success() );
	assert_contains!( out, "cannot overwrite" );

	let svx = read_file("errors/already/main/+page.svx");
	let ts  = read_file("errors/already/main/+page.ts");
	assert_contains!( svx, "shouldn't be overwritten" );
	assert_contains!( ts,  "Don't overwrite me" );
}

#[test] fn source_snapshot_in_errors()
{
	let (status, out) = capture_squarkup_from("errors/source-snapshot");
	assert_not!( status.success() );
	// error messages
	assert_contains!( out, "3 errors while" );
	assert_contains!( out, "unknown twin squark" );
	assert_contains!( out, "unknown squark pattern" );
	assert_contains!( out, "unpaired closing squark" );
	// basic mention
	assert_contains!( out, "<!-- #SQUARK unknown? -->" );
	assert_contains!( out, "<!-- #SQUARK unknown!? -->" );
	assert_contains!( out, "<!-- #SQUARK leave. -->" );
	// line numbers
	assert_contains!( out, " 8 " );
	assert_contains!( out, " 9 " );
	assert_contains!( out, " 10 " );
	// count mentions
	assert_ge!( out.match_indices("<!-- #SQUARK unknown? -->").count(), 2);
	assert_ge!( out.match_indices("<!-- #SQUARK unknown!? -->").count(), 3);
	assert_ge!( out.match_indices("<!-- #SQUARK leave. -->").count(), 2);
}
