use regex::regex;

use super::*;
use crate::core::*;
use crate::utils;
use crate::colours::*;
use crate::macros::*;

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
				folder: dir!(site / "src/routes/"),
				file: str!("+page.svx"),
				page_ts: true,
				data: None,
			},
			format: FormatConfig {
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
	/// Returns `Err(SquarkError::Multiple)` only if nonzero errors are encountered.
	pub fn try_from_toml(data: toml::Table, root: &Path) -> SquarkResult<Self>
	{
		/* Crikey, who knew reading in a config would be such a nightmare... I guess if we want to robustly cover every error path with *user-friendly*, *aggregated* error messages (rather than just a schema violation) we have to handroll it all ourselves */

		/* NOTE:
			We're treating an invalid config as fatal, so `errors.on_error` doesn't apply here. Better to make sure Squarkdown does exactly what the user asks, rather than proceed with misconfigured settings (not that Squarkdown does anything *destructive*, tho)
			
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
				hints!("try setting {W}```\n\n\t[paths]\n\tsite = '/your-site/'\n\n```")
			)?;

			if let Some(value) = paths.get("site") {
				let dir = Self::try_get_string(value, "paths.site", "(filepath relative to your project root)")?;
				site = Self::try_resolve_folder(root, dir, "for your SvelteKit site", fmt!("{W}paths.site{G} is relative to your project root"))?;
			}
		}

		// now start with defaults...
		let mut s = Self::init_defaults(root, &site);

		// ...then apply the user's non-defaults on top of it

		// PathsConfig
		if let Some(paths) = data.get("paths")
		{
			Self::check_is_table(paths, "paths",
				hints!("write your config like this: {W}```\n\n\t[paths]\nsources = ['/']\n\n```")
			)?;

			/* NOTE: We're eagerly clearing defaults... */
			s.paths.sources.clear();

			Self::for_string_array(paths, "paths", "sources", "(filepaths relative to your project root)", &mut errs, |dir, errs| {
				catch!(errs => {
					if dir.contains("/../")
					|| dir.starts_with("..")
					|| dir.ends_with("..")
					{
						return Err(SquarkError::Unrecoverable {
							msg: fmt!("a source folder you specified is invalid: {W}{dir}"),
							hint: fmt!("for safety, {W}..{G} traversal is not allowed in {W}paths.sources"),
							debug: vec![],
						});
					}

					s.paths.sources.push(Self::try_resolve_folder(
						root, dir, "a source folder you specified",
						fmt!("{Y}paths.sources{G} folders are relative from your project root"),
					)?);
				});
			});

			/* NOTE: ...if it turns out `paths.sources` was unset or empty, we'll reinstate the default full-project */
			if s.paths.sources.is_empty() {
				s.paths.sources.push(PathBuf::new());
			}

			Self::for_string_array(paths, "paths", "include", "(RegEx patterns)", &mut errs, |pattern, errs| {
				match regex::Regex::new(pattern) {
					Ok(compiled) => s.paths.include.push(compiled),
					Err(e) => {
						errs.push(SquarkError::External {
							err: bx!(e),
							msg: fmt!("invalid RegEx pattern in {Y}paths.include"),
						});
					}
				}
			});

			Self::for_string_array(paths, "paths", "exclude", "(RegEx patterns)", &mut errs, |pattern, errs| {
				match regex::Regex::new(pattern) {
					Ok(compiled) => s.paths.exclude.push(compiled),
					Err(e) => {
						errs.push(SquarkError::External {
							err: bx!(e),
							msg: fmt!("invalid RegEx pattern in {Y}paths.exclude"),
						});
					}
				}
			});

			// FIXME??
			// if let Some(true) = Self::get_bool(&paths, "paths", "default-exclude", "", &mut errs) {
			// 	s.paths.exclude.clear();
			// }
		}

		// OutConfig
		if let Some(out) = data.get("out")
		{
			Self::check_is_table(out, "out",
				hints!("write your config like this: {W}```\n\n\t[out]\nfile = '+page.svx'\n\n```")
			)?;

			if let Some(value) = out.get("folder") { catch!(errs => {
				let raw = Self::try_get_string(value, "out.folder", "(folder relative to your site folder)")?;
				let dir = Self::try_resolve_folder(&site, raw, "for Squarkdown output", fmt!("{W}out.folder{G} is relative to your site folder"))?;
				s.out.folder = dir;
			}) }

			if let Some(value) = out.get("file") { catch!(errs => {
				let raw = Self::try_get_string(value, "out.file", "(filename including `.svx` extension)")?;
				s.out.file.clone_from(raw);
			}) }

			if let Some(value) = out.get("data") { catch!(errs => {
				let raw = Self::try_get_string(value, "out.data", "(filepath including `.json` extension)")?;
				let path = site.join(utils::to_rel(raw));
				s.out.data = Some(path);
			}) }
		}

		// FormatConfig
		if let Some(format) = data.get("format")
		{
			Self::check_is_table(format, "format",
				hints!("write your config like this: {W}```\n\n\t[format]\n\npreserve-comments = true\n\n```")
			)?;

			let c = &mut s.format;

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

		// FontsConfig

		// ErrorConfig
		if let Some(errors) = data.get("errors")
		{
			Self::check_is_table(errors, "errors",
				hints!("write your config like this: {W}```\n\n\t[errors]\n\non-error = 'kill'\n\n```")
			)?;
			
			if let Some(value) = errors.get("strict") { catch!(errs => {
				s.errors.strict = Self::try_get_bool(value, "errors.strict")?;
			}) }

			if let Some(value) = errors.get("on-error") { catch!(errs => {
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
			}) }
			
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

		errs.or(s)
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

	/// Validate that `root / dir` exists, and is a folder.
	fn try_resolve_folder(
		root: &Path,
		dir: &str,
		location: &'static str,
		hint: String,
	) -> SquarkResult<PathBuf>
	{
		let path = root.join(utils::to_rel(dir));

		if !path.exists() {
			Err(SquarkError::Unrecoverable {
				msg: fmt!("the folder you specified {location} doesn't exist!"),
				hint,
				debug: vec![
					slash!("`{}` is not a valid directory", path),
				],
			})
		}
		else if !path.is_dir() {
			Err(SquarkError::Unrecoverable {
				msg: fmt!("{location} is not a folder"),
				hint: str!(),
				debug: vec![
					slash!("`{}` is not a folder", path),
				],
			})
		}
		else {
			Ok(path)
		}
	}

	fn for_string_array(
		data: &toml::Value,
		category: &'static str,
		field: &'static str,
		hint: &'static str,
		errs: &mut SquarkError,
		mut callback: impl FnMut(&String, &mut SquarkError),
	)
	{
		match Self::get_string_array(data, category, field, hint)
		{
			Ok(None) => (),
			Ok(Some(values)) => for value in values {
				match Self::require_string_entry(value, category, field, hint) {
					Ok(value) => callback(value, errs),
					Err(e) => { errs.push(e); }
				}
			}
			Err(e) => { errs.push(e); }
		}
	}

	/// Validate that `data[field]` is an array.
	fn get_string_array<'d>(
		data: &'d toml::Value,
		category: &'static str,
		field: &'static str,
		hint: &'static str,
	) -> SquarkResult<Option<&'d Vec<toml::Value>>>
	{
		match data.get(field)
		{
			Some(toml::Value::Array(values)) => Ok(Some(values)),
			None => Ok(None),

			Some(v) => Err(SquarkError::Unrecoverable {
				msg: fmt!("invalid setting for {Y}{category}.{field}{R}"),
				hint: fmt!("{Y}{category}.{field}{G} must be an array of strings {GREY}{hint}"),
				debug: vec![
					fmt!("you provided {GREY1}{v}{GREY}, which has type: {GREY1}{}{GREY}", v.type_str()),
				],
			}),
		}
	}

	/// Validate that `data` is a string in an array, for `category.field`.
	fn require_string_entry<'d>(
		data: &'d toml::Value,
		category: &'static str,
		field: &'static str,
		hint: &'static str,
	) -> SquarkResult<&'d String>
	{
		match data
		{
			toml::Value::String(value) => Ok(value),

			v => Err(SquarkError::Unrecoverable {
				msg: fmt!("invalid setting for an entry of {Y}{category}.{field}{R}"),
				hint: fmt!("{Y}{category}.{field}{G} entries must be strings {GREY}{hint}"),
				debug: vec![
					fmt!("you provided {GREY1}{v}{GREY}, which has type: {GREY1}{}{GREY}", v.type_str()),
				],
			}),
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
			"[paths]\nsources = ['nonexistent']",
			"[paths]\nsources = ['test-project/nonexistent']",
		] {
			let e = load_config(source).unwrap_err();
			assert_contains!( e, "doesn't exist" );
		}
	}

	#[test] fn reject_sources_with_up() {
		for source in [
			"[paths]\nsources = ['..']",
			"[paths]\nsources = ['../']",
			"[paths]\nsources = ['/..']",
			"[paths]\nsources = ['/../']",
			"[paths]\nsources = ['../illegal']",
			"[paths]\nsources = ['/../illegal']",
			"[paths]\nsources = ['still/../illegal']",
		] {
			let e = load_config(source).unwrap_err();
			assert_contains!( e, "paths.sources" );
			assert_contains!( e, ".." );
		}
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
			let e = load_config(source).unwrap_err();
			assert_contains!( e, "errors.on-error" );
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
