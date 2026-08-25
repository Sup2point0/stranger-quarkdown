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
			bases:  BasesConfig { folder: None, page_js: None },
			styles: StylesConfig { folder: None, base_file: None },
			assets: AssetsConfig { folder: None, site_assets_folder: None,
				extensions: vec![
					str!("png"),
					str!("jpg"),
					str!("jpeg"),
					str!("webp"),
					str!("svg"),
				],
			},
			fonts:  FontsConfig { queries: vec![] },
			errors: ErrorConfig { on_error: ErrorAction::WARN, on_file_exists: FileAction::OVERWRITE },
		}
	}

	/// Construct a `SquarkupConfig` from TOML `data`, with values fully validated.
	/// 
	/// Returns `Err(SquarkError::ManyRecoverable)` only if nonzero errors are encountered.
	pub fn try_from_toml(data: toml::Table, root: &Path) -> SquarkResult<Self>
	{
		let mut errs = vec![];

		/* NOTE: Read `paths.site` first because *defaults* depend on it */
		// TODO cleanup using `error_if_not_string()`
		let site = if let Some(paths) = data.get("paths")
		{
			match paths.get("site") {
				Some(toml::Value::String(dir)) => Self::if_exists(root, dir, "for your SvelteKit site", fmt!("{W}paths.site{G} is relative to your project root"))?,
				Some(v) => return Err(SquarkError::Unrecoverable {
					msg: fmt!("you provided an invalid {Y}paths.site{R} of type {}", v.type_str()),
					hint: fmt!("{W}paths.site{G} must be a string {GREY}(folder relative to root)"),
					debug: vec![],
				}),
				None => root.to_path_buf(),
			}
		} else { root.to_path_buf() };

		// start with defaults...
		let mut s = Self::init_defaults(root, &site);

		// ...then apply the user's non-defaults on top of it
		if let Some(paths) = data.get("paths")
		{
			Self::for_string_array(&paths, "paths", "sources", "(filepaths relative to your project root", &mut errs, |dir, errs| {
				let path = s.paths.root.join(dir.trim_start_matches("/"));

				if path.exists() {
					s.paths.sources.push(path);
				}
				else {
					errs.push(SquarkError::Unrecoverable {
						msg: slash!("a source folder you specified does not exist: {}", path),
						hint: fmt!("{Y}paths.sources{G} folders are relative from your project root"),
						debug: vec![],
					});
				}
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

			if let Some(true) = Self::error_if_not_bool(&paths, "paths", "default-exclude", "", &mut errs) {
				s.paths.exclude.clear();
			}
		}

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

			Some(v) => Err(SquarkError::Recoverable {
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
			v => Err(SquarkError::Recoverable {
				msg: fmt!("invalid setting for an entry of {Y}{category}.{field}{R}"),
				hint: fmt!("{Y}{category}.{field}{G} entries must be strings {GREY}{hint}"),
				debug: vec![
					fmt!("you provided {GREY1}{v}{GREY}, which has type: {GREY1}{}{GREY}", v.type_str()),
				],
			}),
		}
	}

	/// Validate that `data[field]` is a string, for `category.field`.
	fn error_if_not_string<'d>(
		data: &'d toml::Value,
		category: &'static str,
		field: &'static str,
		hint: &'static str,
		errs: &mut Vec<SquarkError>,
	) -> Option<&'d String>
	{
		match data.get(field)
		{
			Some(toml::Value::String(value)) => Some(value),
			None => None,
			Some(v) => {
				errs.push(SquarkError::Recoverable {
					msg: fmt!("invalid setting for an entry of {Y}{category}.{field}{R}"),
					hint: fmt!("{Y}{category}.{field}{G} must be a string {GREY}{hint}"),
					debug: vec![
						fmt!("you provided {GREY1}{v}{GREY}, which has type: {GREY1}{}{GREY}", v.type_str()),
					],
				});
				None
			},
		}
	}

	/// Validate that `data[field]` is a boolean, for `category.field`.
	fn error_if_not_bool(
		data: &toml::Value,
		category: &'static str,
		field: &'static str,
		hint: &'static str,
		errs: &mut Vec<SquarkError>,
	) -> Option<bool>
	{
		match data.get(field)
		{
			Some(toml::Value::Boolean(value)) => Some(*value),
			None => None,
			Some(v) => {
				errs.push(SquarkError::Recoverable {
					msg: fmt!("invalid setting for an entry of {Y}{category}.{field}{R}"),
					hint: fmt!("{Y}{category}.{field}{G} must be a boolean {GREY}{hint}"),
					debug: vec![
						fmt!("you provided {GREY1}{v}{GREY}, which has type: {GREY1}{}{GREY}", v.type_str()),
					],
				});
				None
			},
		}
	}

	fn if_exists(
		root: &Path,
		dir: &str,
		for_location: &'static str,
		hint: String,
	) -> SquarkResult<PathBuf>
	{
		let path = root.join(dir.trim_start_matches(|c| matches!(c, '/' | '\\')));

		if path.exists() {
			Ok(path)
		}
		else {
			Err(SquarkError::Unrecoverable {
				msg: fmt!("the directory you specified {for_location} doesn't exist!"),
				hint,
				debug: vec![
					slash!("`{}` is not a valid directory", path),
				],
			})
		}
	}
}
