use super::*;
use crate::core::*;
use crate::utils;
use crate::colours::*;
use crate::macros::*;
use path_clean::PathClean;

use path_macro::path;
use regex::{ Regex, regex };

use std::path::{ Path, PathBuf };


impl SquarkupConfig
{
	/// Construct a `SquarkupConfig` with defaults applied and *resolved* against `root`.
	/// 
	/// We can't implement `Default` because paths depend on the project `root`, which is only available at runtime!
	#[must_use]
	pub fn init_defaults(root: &Path, site: &Path) -> Self
	{
		/* NOTE: This is the canonical source of truth for Squarkdown's defaults, make sure to sync docs with this! */
		Self {
			paths: PathsConfig {
				root: root.to_path_buf(),
				site: site.to_path_buf(),
				sources: vec![PathBuf::new()],
				include: vec![
					regex!(r"\.md$").clone(),
				],
				exclude: vec![
					regex!(r"/\.git/").clone(),
					regex!(r"/node_modules/").clone(),
					regex!(r"/.svelte-kit/").clone(),
				],
			},
			out: OutConfig {
				folder: path!(site / "src/routes/"),
				file_name: str!("+page.svx"),
				render_page_ts: true,
				shorter_fields: false,
				site_data_path: None,
			},
			format: FormatConfig {
				preserve_heading: false,
				preserve_comments: false,
				externalise_links: false,
				mark_invalid_links: false,
			},
			styles: StylesConfig { folder: None, base_file: None },
			assets: AssetsConfig { folder: None, site_assets_folder: None,
				extensions: vec![
					str!("png"), str!("jpg"), str!("jpeg"), str!("webp"), str!("svg"),
				],
			},
			fonts:  FontsConfig { queries: vec![] },
			errors: ErrorConfig {
				strict: true,
				on_error: ErrorAction::WARN,
				file_already_exists: FileAction::OVERWRITE,
				inactive_link: LinkRewriteAction::STRIP_EXTENSION,
			},
		}
	}

	/// Construct a `SquarkupConfig` from TOML `data`, with values fully validated.
	/// 
	/// Returns [`SquarkError::Multiple`] if any validation errors are encountered.
	pub fn try_from_toml(data: toml::Table, root: &Path) -> SquarkResult<Self>
	{
		/* Crikey, who knew reading in a config would be such a nightmare... I guess if we want to robustly cover every error path with *user-friendly*, *aggregated* error messages (rather than just a schema violation) we have to handroll it all ourselves */

		/* NOTE:
			We're treating an invalid config as fatal, so `errors.on-error` doesn't apply here. Better to make sure Squarkdown does exactly what the user asks, rather than proceed with misconfigured settings (not that Squarkdown does anything *destructive*, tho)
			
			However, better than repeatedly failing with fatal errors is to report all of them at once, so *if possible*, we'll still process the entire config and aggregate any errors we encounter in `errs`, only returning `Err()` once we reach the end.
		*/
		let mut errs = SquarkError::multiple();

		/* NOTE:
			We first separately read `paths.site` because many *defaults* depend on it, so we need it before calling `::init_defaults()`.
			
			Since this may invalidate the relevance fatal errors later on (i.e. they might all be fixed by fixing `paths.site`), if this fails we'll immediately bail.
		*/
		let mut site = root.to_path_buf();

		if let Some(paths) = data.get("paths")
		{
			Self::check_is_table(paths, "paths",
				hints!("try setting {W}```\n\t[paths]\n\tsite = '/your-site/'\n```")
			)?;

			if let Some(value) = paths.get("site") {
				let dir = Self::try_get_string(value, "paths.site", "(filepath relative to your project root)")?;
				site = Self::try_resolve_folder(root, dir, "for your SvelteKit site", hints!("{W}paths.site{G} is relative to your project root"))?;
			}
		}

		// now start with defaults...
		let mut s = Self::init_defaults(root, &site);

		// ...then apply the user's non-defaults on top of it

		// PathsConfig
		if let Some(paths) = data.get("paths")
		{
			Self::check_is_table(paths, "paths",
				hints!("write your config like this: {W}```\n\t[paths]\n\tsources = ['/']\n```")
			)?;

			if let Some(value) = paths.get("sources") { catch!(errs => {
				let values = Self::try_get_array(value, "paths.sources", "(of folders relative to your project root)")?;

				if !values.is_empty() {
					s.paths.sources.clear();
				}

				for value in values {
					let raw = Self::try_get_string(value, "paths.sources", "(entry in an array)")?;
					let dir = Self::try_resolve_folder(
						root, raw, "a source folder you specified",
						hints!("{Y}paths.sources{G} folders are relative from your project root"),
					)?;
					// TODO check rooted
					s.paths.sources.push(dir);
				}
			}) }

			if let Some(value) = paths.get("include") { catch!(errs => {
				let values = Self::try_get_array(value, "paths.include", "(of RegEx patterns)")?;

				if !values.is_empty() {
					s.paths.include.clear();
				}

				for value in values { catch!(errs => {
					let pattern = Self::try_get_string(value, "paths.include", "(entry in an array)")?;

					match Regex::new(pattern) {
						Ok(compiled) => s.paths.include.push(compiled),
						Err(e) => return Err(SquarkError::External {
							err: bx!(e),
							msg: fmt!("invalid RegEx pattern in {Y}paths.include"),
						})
					}
				}) }
			}) }

			if let Some(value) = paths.get("exclude") { catch!(errs => {
				let values = Self::try_get_array(value, "paths.exclude", "(of RegEx patterns)")?;

				for value in values { catch!(errs => {
					let pattern = Self::try_get_string(value, "paths.exclude", "(entry in an array)")?;

					match Regex::new(pattern) {
						Ok(compiled) => s.paths.exclude.push(compiled),
						Err(e) => return Err(SquarkError::External {
							err: bx!(e),
							msg: fmt!("invalid RegEx pattern in {Y}paths.exclude"),
						})
					}
				}) }
			}) }
		}

		// OutConfig
		if let Some(out) = data.get("out")
		{
			Self::check_is_table(out, "out",
				hints!("write your config like this: {W}```\n\t[out]\n\tfile = '+page.svx'\n```")
			)?;

			if let Some(value) = out.get("folder") { catch!(errs => {
				let raw = Self::try_get_string(value, "out.folder", "(folder relative to your SvelteKit site)")?;
				let dir = Self::try_resolve_folder(&site, raw, "for Squarkdown output", hints!("{W}out.folder{G} is relative to your site folder"))?;
				s.out.folder = dir;
			}) }

			if let Some(value) = out.get("file-name") { catch!(errs => {
				let raw = Self::try_get_string(value, "out.file-name", "(filename including `.svx` extension)")?;
				s.out.file_name.clone_from(raw);
			}) }

			if let Some(value) = out.get("render-page-ts") { catch!(errs => {
				s.out.render_page_ts = Self::try_get_bool(value, "out.render-page-ts")?;
			}) }

			if let Some(value) = out.get("shorter-fields") { catch!(errs => {
				if s.out.render_page_ts == false {
					return Err(SquarkError::Recoverable {
						msg: str!("warning: setting {W}out.shorter-fields{R} when {W}out.render-page-ts{R} is disabled does nothing!"),
						hint: str!("did you mean to enable {Y}out.render-page-ts{G} = {W}true{G}?"),
						debug: vec![],
					});
				}
				s.out.shorter_fields = Self::try_get_bool(value, "out.shorter-fields")?;
			}) }

			if let Some(value) = out.get("site-data-path") { catch!(errs => {
				let raw = Self::try_get_string(value, "out.site-data-path", "(filepath including `.json` extension)")?;
				let path = path!(site / utils::to_rel(raw));

				// TODO cleanup with helper?
				let folder = path.parent().expect("site directory always has a parent folder");
				
				if !folder.exists() {
					return Err(SquarkError::Unrecoverable {
						msg: fmt!("the folder you specified for site data to be saved doesn't exist!"),
						hint: fmt!("{W}out.site-data-path{G} is a filepath relative to your site directory"),
						debug: vec![
							str!(slash!("{GREY1}{}{GREY} is not a valid directory", path))
						],
					});
				}

				s.out.site_data_path = Some(path);
			}) }
		}

		// FormatConfig
		if let Some(format) = data.get("format")
		{
			Self::check_is_table(format, "format",
				hints!("write your config like this: {W}```\n\t[format]\n\tpreserve-comments = true\n```")
			)?;

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
			if let Some(value) = format.get("mark-invalid-links") { catch!(errs => {
				c.mark_invalid_links = Self::try_get_bool(value, "format.mark-invalid-links")?;
			}) }
		}

		// StylesConfig

		// AssetsConfig
		if let Some(assets) = data.get("assets")
		{
			Self::check_is_table(assets, "assets",
				hints!("write your config like this: {W}```\n\t[assets]\n\tfolder = '.github/assets'\n```")
			)?;

			if let Some(value) = assets.get("folder") { catch!(errs => {
				let dir = Self::try_get_string(value, "assets.folder", "(folder relative to your project root)")?;
				let folder = Self::try_resolve_folder(root, dir, "for assets", hints!("{W}assets.folder{G} is relative to your project root"))?;

				s.assets.folder = Some(folder);
			}) }
		}

		// FontsConfig

		// ErrorConfig
		if let Some(errors) = data.get("errors")
		{
			Self::check_is_table(errors, "errors",
				hints!("write your config like this: {W}```\n\t[errors]\n\ton-error = 'kill'\n```")
			)?;
			
			if let Some(value) = errors.get("strict") { catch!(errs => {
				s.errors.strict = Self::try_get_bool(value, "errors.strict")?;
			}) }

			/* NOTE: This is the one field that isn't aggregated into `errs`... because all error handling depends on it, so the user _must_ provide a valid value! */
			if let Some(value) = errors.get("on-error") {
				let raw = Self::try_get_string(value, "errors.on-error", "(an error handling strategy)")?;

				if let Ok(opt) = ErrorAction::try_from(raw.as_str()) {
					s.errors.on_error = opt;
				} else {
					return Err(SquarkError::Unrecoverable {
						msg: fmt!("unknown setting for {Y}errors.on-error"),
						hint: fmt!("valid values are {W}'warn'{G} (default) or {W}'kill'"),
						debug: vec![fmt!("you provided {value}")],
					});
				}
			}
			
			if let Some(value) = errors.get("file-already-exists") { catch!(errs => {
				let raw = Self::try_get_string(value, "errors.file-already-exists", "(a file conflict handling strategy)")?;

				if let Ok(opt) = FileAction::try_from(raw.as_str()) {
					s.errors.file_already_exists = opt;
				} else {
					return Err(SquarkError::Unrecoverable {
						msg: fmt!("unknown setting for {Y}errors.file-already-exists"),
						hint: fmt!("valid values are {W}'overwrite'{G} (default), {W}'error'{G}, {W}'skip'"),
						debug: vec![fmt!("you provided {value}")],
					});
				}
			}) }
			
			if let Some(value) = errors.get("inactive-link") { catch!(errs => {
				let raw = Self::try_get_string(value, "errors.inactive-link", "(a missing file handling strategy)")?;

				if let Ok(opt) = LinkRewriteAction::try_from(raw.as_str()) {
					s.errors.inactive_link = opt;
				} else {
					return Err(SquarkError::Unrecoverable {
						msg: fmt!("unknown setting for {Y}errors.inactive-link"),
						hint: fmt!("valid values are {W}'strip-extension'{G} (default), {W}'link-to-github'{G} or {W}'error'"),
						debug: vec![fmt!("you provided {value}")],
					});
				}
			}) }
		}

		let _ = errs.or_depends((), &s)?;
		Ok(s)
	}
}

/// All the validation logic!
impl SquarkupConfig
{
	/// Validate that `data` is a TOML table.
	fn check_is_table(data: &toml::Value, setting: &str, hint: impl FnOnce() -> String) -> SquarkResult
	{
		if matches!(data, toml::Value::Table(..)) {
			Ok(())
		}
		else {
			Err(SquarkError::Unrecoverable {
				msg: fmt!("{Y}{setting}{R} must be a table, not a field"),
				hint: hint(),
				debug: vec![
					fmt!("you provided {GREY1}{data}{GREY}, which has type {GREY1}{}{GREY}", data.type_str()),
				],
			})
		}
	}

	/// Try to extract the string from `data` for `setting`.
	fn try_get_string<'d>(
		value: &'d toml::Value,
		setting: &str,
		hint: &'static str,
	) -> SquarkResult<&'d String>
	{
		match value {
			toml::Value::String(v) => Ok(v),

			v => Err(SquarkError::Unrecoverable {
				msg: fmt!("invalid setting for an entry of {Y}{setting}{R}"),
				hint: fmt!("{Y}{setting}{G} must be a string {GREY}{hint}"),
				debug: vec![
					fmt!("you provided {GREY1}{v}{GREY}, which has type: {GREY1}{}{GREY}", v.type_str()),
				],
			}),
		}
	}

	/// Try to extract the boolean from `data` for `setting`.
	fn try_get_bool(value: &toml::Value, setting: &str) -> SquarkResult<bool>
	{
		match value {
			toml::Value::Boolean(v) => Ok(*v),
			
			v => Err(SquarkError::Unrecoverable {
				msg: fmt!("invalid setting for an entry of {Y}{setting}{R}"),
				hint: fmt!("{Y}{setting}{G} must be a boolean"),
				debug: vec![
					fmt!("you provided {GREY1}{v}{GREY}, which has type: {GREY1}{}{GREY}", v.type_str()),
				],
			}),
		}
	}

	/// Try to extract the array from `data` for `setting`.
	fn try_get_array<'d>(
		value: &'d toml::Value,
		setting: &str,
		hint: &'static str,
	) -> SquarkResult<&'d [toml::Value]>
	{
		match value {
			toml::Value::Array(v) => Ok(v),
			
			v => Err(SquarkError::Unrecoverable {
				msg: fmt!("invalid setting for an entry of {Y}{setting}{R}"),
				hint: fmt!("{Y}{setting}{G} must be an array {GREY}{hint}"),
				debug: vec![
					fmt!("you provided {GREY1}{v}{GREY}, which has type: {GREY1}{}{GREY}", v.type_str()),
				],
			}),
		}
	}

	/// Validate that `root / dir` exists, and is a folder.
	fn try_resolve_folder(
		root: &Path,
		dir: &str,
		location: &'static str,
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

		assert_eq!( c.folder, Some(path!(*TESTS / "static")) );
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
mod error_handling {
	use super::*;

	#[test] fn accept() {
		let c = load_config(indoc! {"
			[errors]
			strict = true
			on-error = 'kill'
			file-already-exists = 'error'
			inactive-link = 'error'
		"}).unwrap().errors;

		assert_eq!( c.strict, true );
		assert_eq!( c.on_error, ErrorAction::KILL );
		assert_eq!( c.file_already_exists, FileAction::ERROR );
		assert_eq!( c.inactive_link, LinkRewriteAction::ERROR );
		
		let c = load_config(indoc! {"
			[errors]
			strict = false
			on-error = 'warn'
			file-already-exists = 'overwrite'
			inactive-link = 'strip-extension'
		"}).unwrap().errors;

		assert_eq!( c.strict, false );
		assert_eq!( c.on_error, ErrorAction::WARN );
		assert_eq!( c.file_already_exists, FileAction::OVERWRITE );
		assert_eq!( c.inactive_link, LinkRewriteAction::STRIP_EXTENSION );
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
