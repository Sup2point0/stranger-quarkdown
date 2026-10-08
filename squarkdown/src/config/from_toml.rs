use super::*;
use crate::prelude::*;
use crate::utils;
use crate::colours::*;
use crate::macros::*;

use path_clean::PathClean;
use path_macro::path;
use regex::{ Regex };

use std::path::{ Path, PathBuf };


/// Lazily produce a string for a hint message.
macro_rules! hints
{
	() => {
		|| String::new()
	};
	($($args:tt)*) => {
		|| format!($($args)*)
	};
}

/// Return a [`SquarkError::Unrecoverable`].
macro_rules! unrecoverable
{
	($($args:tt)*) => {
		return Err(SquarkError::Unrecoverable { $($args)* })
	}
}


/// Loading from `squarkup.toml`
impl SquarkupConfig
{
	/// Construct a `SquarkupConfig` from TOML `data`, with values fully validated.
	/// 
	/// Returns [`SquarkError::Multiple`] if any validation errors are encountered.
	pub fn try_from_toml(data: toml::Table, root: &Path) -> SquarkResult<Self>
	{
		/* Crikey, who knew reading in a config would be such a nightmare... I guess if we want to robustly cover every error path with _user-friendly_, _aggregated_ error messages (rather than just a schema violation) we have to handroll it all ourselves */

		/* NOTE:
			We're treating an invalid config as fatal, so `errors.on-error` doesn't apply here. Better to make sure Squarkdown does exactly what the user asks, rather than proceed with misconfigured settings (not that Squarkdown does anything _destructive_, tho)
			
			However, better than repeatedly failing with fatal errors is to report all of them at once, so _if possible_, we'll still process the entire config and aggregate any errors we encounter in `errs`, only returning `Err()` once we reach the end.
		*/
		let mut errs = SquarkError::multiple("loading squarkup config");

		/* NOTE:
			We first separately read `paths.site` because many _defaults_ depend on it, so we need it before calling `::init_defaults()`.
			
			Since this may invalidate the relevant fatal errors later on (i.e. they might all be fixed by fixing `paths.site`), if this fails we'll immediately bail.
		*/
		let mut site = root.to_path_buf();

		if let Some(paths) = Self::try_get_table_if_present(&data, "paths",
			hints!("write your config like this: {W}```\n\t[paths]\n\tsite = '/your-site/'\n```")
		)?
		{
			if let Some(value) = paths.get("site") {
				let dir = Self::try_get_str(value, "paths.site", "(filepath relative to your project root)")?;
				site = Self::try_resolve_folder(root, dir, "for your SvelteKit site", hints!("{W}paths.site{G} is relative to your project root"))?;
			}
		}

		// now start with defaults...
		let mut s = Self::init_defaults(root, &site);
		// ...then apply the user's non-defaults on top of it

		// == ErrorConfig == //
		if let Some(errors) = Self::try_get_table_if_present(&data, "errors",
			hints!("write your config like this: {W}```\n\t[errors]\n\ton-error = 'kill'\n```")
		)?
		{
			if let Some(value) = errors.get("strict") { catch!(errs => {
				s.errors.strict = Self::try_get_bool(value, "errors.strict")?;
			}) }

			/* NOTE: This is the one field that isn't aggregated into `errs`... because all error handling depends on it, so the user _must_ provide a valid value! */
			if let Some(value) = errors.get("on-error") {
				let raw = Self::try_get_str(value, "errors.on-error", "(an error handling strategy)")?;

				if let Ok(opt) = ErrorAction::try_from(raw) {
					s.errors.on_error = opt;
				} else {
					unrecoverable! {
						msg: fmt!("unknown setting for {Y}errors.on-error"),
						hint: fmt!("valid values are {W}'warn'{G} (default) or {W}'kill'"),
						debug: vec![fmt!("you provided {value}")],
					}
				}
			}
			
			if let Some(value) = errors.get("file-already-exists") { catch!(errs => {
				let raw = Self::try_get_str(value, "errors.file-already-exists", "(a file conflict handling strategy)")?;

				if let Ok(opt) = FileAction::try_from(raw) {
					s.errors.file_already_exists = opt;
				} else {
					unrecoverable! {
						msg: fmt!("unknown setting for {Y}errors.file-already-exists"),
						hint: fmt!("valid values are {W}'overwrite'{G} (default), {W}'error'{G}, {W}'skip'"),
						debug: vec![fmt!("you provided {value}")],
					}
				}
			}) }
			
			if let Some(value) = errors.get("broken-link") { catch!(errs => {
				let raw = Self::try_get_str(value, "errors.broken-link", "(a missing file handling strategy)")?;

				if let Ok(opt) = LinkRewriteAction::try_from(raw) {
					s.errors.link_broken = opt;
				} else {
					unrecoverable! {
						msg: fmt!("unknown setting for {Y}errors.broken-link"),
						hint: fmt!("valid values are {W}'strip-extension'{G} (default), {W}'link-to-github'{G} or {W}'error'"),
						debug: vec![fmt!("you provided {value}")],
					}
				}
			}) }
		}

		// == RepoConfig == //
		if let Some(project) = Self::try_get_table_if_present(&data, "repo",
			hints!("write your config like this: {W}```\n\t[repo]\n\tgithub = 'Sup2point0/stranger-quarkdown'\n```")
		)?
		{
			if let Some(value) = project.get("name") { catch!(errs => {
				let raw = Self::try_get_str(value, "project.name", "(displayed name of project)")?;

				if raw.is_empty() {
					return Err(SquarkError::Recoverable {
						msg: fmt!("warning: you provided an empty {Y}project.name"),
						hint: str!("project name is ignored if empty"),
						debug: vec![],
					});
				}

				raw.clone_into(&mut s.project.name);
			}) }
			
			if let Some(value) = project.get("github") { catch!(errs => {
				let raw = Self::try_get_str(value, "project.github",
					&fmt!("(GitHub repo link in {W}user/repo{G} format)")
				)?;

				if raw.is_empty() {
					return Err(SquarkError::Recoverable {
						msg: fmt!("warning: you provided an empty {Y}project.github"),
						hint: str!("project GitHub link is ignored if empty"),
						debug: vec![],
					});
				}
				else if !raw.contains("/") {
					errs.push(SquarkError::Recoverable {
						msg: fmt!("warning: your {Y}project.github{R} does not contain a {W}/"),
						hint: fmt!("use the format {W}user/project"),
						debug: vec![
							fmt!("you provided {GREY1}{raw}"),
						],
					});
				}

				raw.clone_into(&mut s.project.github);
			}) }
		}

		// == PathsConfig == //
		if let Some(paths) = Self::try_get_table_if_present(&data, "paths", hints!())?
		{
			if let Some(value) = paths.get("sources") { catch!(errs => {
				let values = Self::try_get_array(value, "paths.sources", "(of folders relative to your project root)")?;

				if !values.is_empty() {
					s.paths.sources.clear();
				}

				for value in values {
					let raw = Self::try_get_str(value, "paths.sources", "(entry in an array)")?;
					let dir = Self::try_resolve_folder(
						root, raw, "a source folder you specified",
						hints!("{Y}paths.sources{G} folders are relative from your project root"),
					)?;
					s.paths.sources.push(dir);
				}
			}) }

			if let Some(value) = paths.get("include") { catch!(errs => {
				let values = Self::try_get_array(value, "paths.include", "(of RegEx patterns)")?;

				if !values.is_empty() {
					s.paths.include.clear();
				}

				for value in values { catch!(errs => {
					let pattern = Self::try_get_str(value, "paths.include", "(RegEx pattern)")?;

					match Regex::new(pattern) {
						Ok(compiled) => s.paths.include.push(compiled),
						Err(e) => return Err(SquarkError::External {
							err: Box::new(e),
							msg: fmt!("invalid RegEx pattern in {Y}paths.include"),
						})
					}
				}) }
			}) }

			if let Some(value) = paths.get("exclude") { catch!(errs => {
				let values = Self::try_get_array(value, "paths.exclude", "(of RegEx patterns)")?;

				for value in values { catch!(errs => {
					let pattern = Self::try_get_str(value, "paths.exclude", "(RegEx pattern)")?;

					match Regex::new(pattern) {
						Ok(compiled) => s.paths.exclude.push(compiled),
						Err(e) => return Err(SquarkError::External {
							err: Box::new(e),
							msg: fmt!("invalid RegEx pattern in {Y}paths.exclude"),
						})
					}
				}) }
			}) }
		}

		// == OutConfig == //
		if let Some(out) = Self::try_get_table_if_present(&data, "out",
			hints!("write your config like this: {W}```\n\t[out]\n\tfile = '+page.svx'\n```")
		)?
		{
			if let Some(value) = out.get("folder") { catch!(errs => {
				let raw = Self::try_get_str(value, "out.folder", "(folder relative to your SvelteKit site)")?;
				let dir = Self::try_resolve_folder(&site, raw, "for Squarkdown output", hints!("{W}out.folder{G} is relative to your site folder"))?;
				s.out.folder = dir;
			}) }

			if let Some(value) = out.get("file-name") { catch!(errs => {
				let raw = Self::try_get_str(value, "out.file-name", &fmt!("(filename including {GREY1}.svx{GREY} extension)"))?;

				if raw.contains('/') {
					unrecoverable! {
						msg: fmt!("illegal value for {Y}out.file-name{R}: {W}{raw}"),
						hint: fmt!("the file name cannot contain {W}/{G}, because that turns into a file path!"),
						debug: vec![],
					}
				}

				raw.clone_into(&mut s.out.file_name);
			}) }

			if let Some(value) = out.get("site-data-path") { catch!(errs => {
				let raw = Self::try_get_str(value, "out.site-data-path", "(filepath including `.json` extension)")?;

				let path = path!(site / utils::to_rel(raw));

				let folder = path.parent().expect("site directory always has a parent folder");

				if !folder.exists() {
					unrecoverable! {
						msg: fmt!("the folder you specified for site data to be saved doesn't exist!"),
						hint: fmt!("{W}out.site-data-path{G} is a filepath relative to your site directory"),
						debug: vec![
							slash!("{GREY1}{}{GREY} is not a valid directory", path)
						],
					}
				}

				s.out.site_data_path = Some(path);
			}) }

			if let Some(value) = out.get("render-page-ts") { catch!(errs => {
				s.out.render_page_ts = Self::try_get_bool(value, "out.render-page-ts")?;
			}) }

			if let Some(value) = out.get("shorter-fields") { catch!(errs => {
				s.out.shorter_fields = Self::try_get_bool(value, "out.shorter-fields")?;
			}) }
		}

		// == FormatConfig == //
		if let Some(format) = Self::try_get_table_if_present(&data, "format",
			hints!("write your config like this: {W}```\n\t[format]\n\tpreserve-comments = true\n```")
		)?
		{
			let c = &mut s.format;

			if let Some(value) = format.get("preserve-heading") { catch!(errs => {
				c.preserve_heading = Self::try_get_bool(value, "format.preserve-heading")?;
			}) }
			if let Some(value) = format.get("preserve-comments") { catch!(errs => {
				c.preserve_comments = Self::try_get_bool(value, "format.preserve-comments")?;
			}) }
			if let Some(value) = format.get("externalise-links") { catch!(errs => {
				c.externalise_links = Self::try_get_bool(value, "format.externalise-links")?;
			}) }
		}

		// == AssetsConfig == //
		if let Some(assets) = Self::try_get_table_if_present(&data, "assets",
			hints!("write your config like this: {W}```\n\t[assets]\n\tfolder = '.github/assets'\n```")
		)?
		{
			if let Some(value) = assets.get("folder") { catch!(errs => {
				let dir = Self::try_get_str(value, "assets.folder", "(folder relative to your project root)")?;
				let folder = Self::try_resolve_folder(root, dir, "for assets", hints!("{Y}assets.folder{G} is relative to your project root"))?;

				s.assets.folder = folder;
			}) }

			if let Some(value) = assets.get("site-assets-folder") { catch!(errs => {
				let dir = Self::try_get_str(value, "assets.site-assets-folder", "(folder relative to your project root)")?;
				let folder = Self::try_resolve_folder(root, dir, "for site assets", hints!("{Y}assets.site-assets-folder{G} is relative to your project root"))?;

				s.assets.site_assets_folder = Some(folder);
			}) }

			if let Some(value) = assets.get("extensions") { catch!(errs => {
				let values = Self::try_get_array(value, "assets.extensions", "(of file extensions without .)")?;

				if !values.is_empty() {
					s.assets.extensions.clear();
				}

				for value in values { catch!(errs => {
					let mut raw = Self::try_get_str(value, "assets.extensions", "(file extension)")?;

					if raw.starts_with('.') {
						raw = &raw[1..];
					}
					
					s.assets.extensions.push(raw.to_owned());
				}) }
			}) }
		}

		// == FontsConfig == //
		if let Some(fonts) = Self::try_get_table_if_present(&data, "fonts",
			hints!("write your config like this: {W}```\n\t[fonts]\n\tqueries = ['Sora:wght@100..800']\n```")
		)?
		{
			if let Some(value) = fonts.get("queries") { catch!(errs => {
				let values = Self::try_get_array(value, "fonts.queries", "(of font query parameters)")?;

				if !values.is_empty() {
					s.fonts.queries.clear();
				}

				for value in values { catch!(errs => {
					let raw = Self::try_get_str(value, "fonts.queries", "(font query parameter)")?;
					
					s.fonts.queries.push(raw.to_owned());
				}) }
			}) }
		}

		errs.depends(&s)?;
		Ok(s)
	}
}

/// All the validation logic!
impl SquarkupConfig
{
	/// Try to extract the table from `data` for `setting-group`.
	/// 
	/// It is fine is `data` does not have `setting-group`, in which case this returns `Ok(None)`.
	fn try_get_table_if_present<'d>(
		data: &'d toml::Table,
		setting_group: &str,
		hint: impl FnOnce() -> String,
	) -> SquarkResult<Option<&'d toml::Table>>
	{
		let Some(value) = data.get(setting_group) else {
			return Ok(None);
		};

		let Some(table) = value.as_table() else {
			unrecoverable! {
				msg: fmt!("{Y}{setting_group}{R} must be a table, not a field"),
				hint: hint(),
				debug: vec![
					fmt!("you provided {GREY1}{data}{GREY}, which has type {GREY1}{}", value.type_str()),
				],
			}
		};

		Ok(Some(table))
	}

	/// Try to extract the string from `data` for `setting`.
	fn try_get_str<'d>(
		value: &'d toml::Value,
		setting: &str,
		hint: &str,
	) -> SquarkResult<&'d str>
	{
		value.as_str()
			.ok_or_else(|| SquarkError::Unrecoverable {
				msg: fmt!("invalid value {W}{value}{R} for {Y}{setting}"),
				hint: fmt!("{Y}{setting}{G} must be a string {GREY}{hint}"),
				debug: vec![
					fmt!("you provided a value of type {GREY1}{}", value.type_str()),
				],
			})
	}

	/// Try to extract the boolean from `data` for `setting`.
	fn try_get_bool(value: &toml::Value, setting: &str) -> SquarkResult<bool>
	{
		value.as_bool()
			.ok_or_else(|| SquarkError::Unrecoverable {
				msg: fmt!("invalid value {W}{value}{R} for {Y}{setting}"),
				hint: fmt!("{Y}{setting}{G} must be a boolean"),
				debug: vec![
					fmt!("you provided a value of type {GREY1}{}", value.type_str()),
				],
			})
	}

	/// Try to extract the array from `data` for `setting`.
	fn try_get_array<'d>(
		value: &'d toml::Value,
		setting: &str,
		hint: &str,
	) -> SquarkResult<&'d Vec<toml::Value>>
	{
		value.as_array()
			.ok_or_else(|| SquarkError::Unrecoverable {
				msg: fmt!("invalid value {W}{value}{R} for {Y}{setting}"),
				hint: fmt!("{Y}{setting}{G} must be an array {GREY}{hint}"),
				debug: vec![
					fmt!("you provided a value of type {GREY1}{}", value.type_str()),
				],
			})
	}

	/// Validate that `root / dir` exists, and is a folder.
	fn try_resolve_folder(
		root: &Path,
		dir: &str,
		location: &str,
		hint: impl FnOnce() -> String,
	) -> SquarkResult<PathBuf>
	{
		let path = path!(root / utils::to_rel(dir));

		if !path.exists() {
			Err(SquarkError::Unrecoverable {
				msg: fmt!("the folder you specified {location} doesn't exist!"),
				hint: hint(),
				debug: vec![
					slash!("{GREY1}{}{GREY} is not a valid directory", path),
				],
			})
		}
		else if !path.is_dir() {
			Err(SquarkError::Unrecoverable {
				msg: fmt!("the folder you specified {location} is not a folder"),
				hint: str!(),
				debug: vec![
					slash!("{GREY1}{}{GREY} is not a folder", path),
				],
			})
		}
		else {
			Ok(path.clean())
		}
	}
}


// == TESTS == //

#[cfg(test)] use crate::utils::testing::*;

#[cfg(test)] use assertables::*;
#[cfg(test)] use indoc::indoc;


#[cfg(test)]
fn load_config(source: &str) -> SquarkResult<SquarkupConfig>
{
	let toml = str!(source).parse::<toml::Table>().unwrap();
	SquarkupConfig::try_from_toml(toml, &TESTS)
}


#[cfg(test)]
mod error_handling {
	use super::*;

	#[test] fn accept() {
		let c = load_config(indoc! {"
			[errors]
			strict = true
			on-error = 'kill'
			file-already-exists = 'error'
			broken-link = 'error'
		"}).unwrap().errors;

		assert_eq!( c.strict, true );
		assert_eq!( c.on_error, ErrorAction::KILL );
		assert_eq!( c.file_already_exists, FileAction::ERROR );
		assert_eq!( c.link_broken, LinkRewriteAction::ERROR );
		
		let c = load_config(indoc! {"
			[errors]
			strict = false
			on-error = 'warn'
			file-already-exists = 'overwrite'
			broken-link = 'strip-extension'
		"}).unwrap().errors;

		assert_eq!( c.strict, false );
		assert_eq!( c.on_error, ErrorAction::WARN );
		assert_eq!( c.file_already_exists, FileAction::OVERWRITE );
		assert_eq!( c.link_broken, LinkRewriteAction::STRIP_EXTENSION );
	}

	#[test] fn reject() {
		for source in [
			"[errors]\non-error = 0",
			"[errors]\non-error = false",
			"[errors]\non-error = 'x'",
			"[errors]\non-error = \"y\"",
		] {
			let e = load_config(source);
			assert_err!( &e );
			assert_contains!( e.unwrap_err(), "errors.on-error" );
		}
	}
}

#[cfg(test)]
mod paths {
	use super::*;

	#[test] fn reject_nonexistent_sources() {
		for source in [
			"[paths]\nsources = ['nonexistent']\n\t[errors]\non-error='kill'",
			"[paths]\nsources = ['test-project/nonexistent']\n\t[errors]\non-error='kill'",
		] {
			let e = load_config(source);
			assert_err!( &e );
			assert_contains!( e.unwrap_err(), "doesn't exist" );
		}
	}
}

#[cfg(test)]
mod assets {
	use super::*;

	#[test] fn accept() {
		let c = load_config(indoc! {"
			[assets]
			folder = 'static'
		"}).unwrap().assets;
		assert_eq!( c.folder, path!(*TESTS / "static") );
	}

	#[test] fn reject() {
		let e = load_config(indoc! {"
			[assets]
			folder = 'nonexistent'
		"});
		assert_err!( &e );
		assert_contains!( e.unwrap_err(), "assets.folder" );
	}
}

#[cfg(test)]
mod fonts {
	use super::*;

	#[test] fn accept() {
		let c = load_config(indoc! {"
			[fonts]
			queries = [
				'Montserrat',
				'Rajdhani',
				'Sora:wght@100..800',
			]
		"}).unwrap().fonts;
		assert_eq!( c.queries, vec!["Montserrat", "Rajdhani", "Sora:wght@100..800"] );
	}
}

#[cfg(test)]
mod rejects {
	use super::*;

	#[test] fn non_tables() {
		for source in [
			"paths = false",
			"out = false",
			"format = false",
			"errors = false",
		] {
			let r = load_config(source);
			assert_err!( &r );
			assert_contains!( r.unwrap_err(), "table" );
		}
	}
}
