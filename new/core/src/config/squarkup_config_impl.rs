use regex::regex;

use super::*;
use crate::{
	SquarkResult, SquarkError,
	colours::*,
	macros::*,
};

use std::path::{ Path, PathBuf };


impl SquarkupConfig
{
	/// Construct a `SquarkupConfig` with defaults applied and *resolved* against `root`.
	/// 
	/// We can't implement `Default` because paths depend on the project `root`, which is only available at runtime!
	pub fn init_defaults(root: &Path, site: &Path) -> Self
	{
		/* NOTE: This is the canonical source of truth for Squarkdown's defaults, make sure to sync docs with this! */
		Self {
			paths: PathsConfig {
				root: root.to_path_buf(),
				site: site.to_path_buf(),
				sources: vec![root.to_path_buf()],
				include: vec![
					regex!(r"\.md$").clone(),
					regex!(r"\.svx$").clone(),
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
			},
			data: DataConfig {
				path: dir!(site / "src/site-data.json")
			},
			format: FormatConfig { preserve_comments: false, externalise_links: false },
			bases:  BasesConfig  { folder: None, page_js: None },
			styles: StylesConfig { folder: None, base_file: None },
			assets: AssetsConfig { folder: None, site_assets_folder: None,
				extensions: vec![
					str!("png"), str!("jpg"), str!("jpeg"), str!("webp"), str!("svg"),
				],
			},
			fonts:  FontsConfig { queries: vec![] },
			errors: ErrorConfig { on_error: ErrorAction::WARN, on_file_exists: FileAction::OVERWRITE },
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
		let mut errs = vec![];

		/* NOTE:
			We first separately read `paths.site` because many *defaults* depend on it, so we need it before calling `::init_defaults()`.
			
			Since this may invalidate the relevance fatal errors later on (i.e. they might all be fixed by fixing `paths.site`), if this fails we'll immediately bail.
		*/
		let mut site = root.to_path_buf();

		if let Some(paths) = data.get("paths")
		{
			Self::check_is_table(paths, "paths", fmt!("try setting {W}```\n\n\t[paths]\n\tsite = \"/your-site/\"\n\n```"))?;

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
			Self::for_string_array(&paths, "paths", "sources", "(filepaths relative to your project root)", &mut errs, |dir, errs| {
				catch!(errs => {
					s.paths.sources.push(Self::try_resolve_folder(
						root, dir, "a source folder you specified",
						fmt!("{Y}paths.sources{G} folders are relative from your project root"),
					)?);
				});
			});

			Self::for_string_array(&paths, "paths", "include", "(RegEx patterns)", &mut errs, |pattern, errs| {
				match regex::Regex::new(&pattern) {
					Ok(compiled) => s.paths.include.push(compiled),
					Err(e) => errs.push(SquarkError::External {
						err: bx!(e),
						msg: fmt!("invalid RegEx pattern in {Y}paths.include"),
					}),
				}
			});

			Self::for_string_array(&paths, "paths", "exclude", "(RegEx patterns)", &mut errs, |pattern, errs| {
				match regex::Regex::new(&pattern) {
					Ok(compiled) => s.paths.exclude.push(compiled),
					Err(e) => errs.push(SquarkError::External {
						err: bx!(e),
						msg: fmt!("invalid RegEx pattern in {Y}paths.exclude"),
					}),
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
			if let Some(value) = out.get("folder") {
				catch!(errs => {
					let raw = Self::try_get_string(value, "out.folder", "(folder relative to your site folder)")?;
					let dir = Self::try_resolve_folder(&site, raw, "for Squarkdown output", fmt!("{W}out.folder{G} is relative to your site folder"))?;
					s.out.folder = dir;
				});
			}

			if let Some(value) = out.get("file") {
				catch!(errs => {
					let raw = Self::try_get_string(value, "out.file", "(filename including `.svx` extension)")?;
					s.out.file = raw.clone();
				});
			}
		}

		// DataConfig

		// FormatConfig
		if let Some(format) = data.get("format")
		{
			if let Some(value) = format.get("preserve-comments") {
				catch!(errs => {
					let raw = Self::try_get_bool(value, "format.preserve-comments")?;
					s.format.preserve_comments = raw.clone();
				});
			}
			
			if let Some(value) = format.get("externalise-links") {
				catch!(errs => {
					let raw = Self::try_get_bool(value, "format.externalise-links")?;
					s.format.externalise_links = raw.clone();
				});
			}
		}

		// BasesConfig

		// StylesConfig

		// AssetsConfig

		// FontsConfig

		// ErrorConfig
		if let Some(errors) = data.get("errors")
		{
			Self::check_is_table(errors, "errors", fmt!("write your config like this: {W}```\n\n\t[errors]\n\non-error = \"kill\"\n\n```"))?;

			if let Some(value) = errors.get("on-error")
			{
				catch!(errs => {
					let raw = Self::try_get_string(value, "errors.on-error", "(an error handling strategy)")?;

					match ErrorAction::try_from(raw.as_str())
					{
						Ok(opt) => s.errors.on_error = opt,
						Err(..) => return Err(SquarkError::Unrecoverable {
							msg: fmt!("unknown setting for {Y}errors.on-error"),
							hint: fmt!("valid values are \"warn\" (default) or \"kill\""),
							debug: vec![
								fmt!("you provided \"{value}\""),
							],
						}),
					}
				});
			}
			
			if let Some(value) = errors.get("on-file-exists")
			{
				catch!(errs => {
					let raw = Self::try_get_string(value, "errors.on-file-exists", "(a file conflict handling strategy)")?;

					match FileAction::try_from(raw.as_str())
					{
						Ok(opt) => s.errors.on_file_exists = opt,
						Err(..) => return Err(SquarkError::Unrecoverable {
							msg: fmt!("unknown setting for {Y}errors.on-file-exists"),
							hint: fmt!("valid values are \"overwrite\" (default), \"error\" or \"skip\""),
							debug: vec![
								fmt!("you provided \"{value}\""),
							],
						}),
					}
				});
			}
		}

		if errs.is_empty() {
			Ok(s)
		} else {
			Err(SquarkError::Multiple { errs })
		}
	}
}

/// All the validation logic!
impl SquarkupConfig
{
	/// Validate that `data` is a TOML table.
	fn check_is_table(data: &toml::Value, setting: &str, hint: String) -> SquarkResult
	{
		if matches!(data, toml::Value::Table(..)) {
			Ok(())
		}
		else {
			Err(SquarkError::Unrecoverable {
				msg: fmt!("{Y}{setting}{R} must be a table, not a field"),
				hint,
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
		let path = root.join(dir.trim_start_matches(|c| matches!(c, '/' | '\\')));

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

	fn for_string_array<'d>(
		data: &'d toml::Value,
		category: &'static str,
		field: &'static str,
		hint: &'static str,
		errs: &mut Vec<SquarkError>,
		mut callback: impl FnMut(&String, &mut Vec<SquarkError>),
	)
	{
		match Self::get_string_array(data, category, field, hint)
		{
			Ok(None) => (),
			Ok(Some(values)) => for value in values {
				match Self::require_string_entry(value, category, field, hint) {
					Ok(value) => callback(value, errs),
					Err(e) => errs.push(e),
				}
			},
			Err(e) => errs.push(e),
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
