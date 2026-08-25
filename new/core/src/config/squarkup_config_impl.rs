use regex::regex;

use super::*;
use crate::{
	SquarkResult, SquarkError,
	colours::*,
	macros::*,
};

use std::path::{ Path };


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
		let site = if let Some(paths) = data.get("paths")
		{
			match paths.get("site") {
				Some(toml::Value::String(dir)) => {
					let path = root.join(dir.trim_start_matches("/"));

					if !path.exists() {
						return Err(SquarkError::Unrecoverable {
							msg: str!("the directory you specified for your SvelteKit site doesn't exist!"),
							hint: format!("{WHITE}paths.site{GREEN} is relative from your project root"),
							debug: vec![slash!("{GREY_LIGHT}{}{GREY} is not a valid directory", path)],
						});
					}

					path
				},
				Some(v) => return Err(SquarkError::Unrecoverable {
					msg: format!("you provided an invalid {YELLOW}paths.site{RED} of type {}", v.type_str()),
					hint: format!("{WHITE}paths.site{GREEN} must be a string {GREY}(folder relative to root)"),
					debug: vec![],
				}),
				None => root.to_path_buf(),
			}
		} else { root.to_path_buf() };

		// start with defaults...
		let mut out = Self::init_defaults(root, &site);

		// ...then apply the user's non-defaults on top of it
		if let Some(paths) = data.get("paths")
		{
			if let Some(values) = Self::get_string_array(&paths, "paths", "sources", "(filepaths relative to your project root", &mut errs) {
				for value in values {
					if let Some(dir) = Self::require_string_entry(value, "paths", "sources", "(filepaths relative to your project root)", &mut errs) {
						let path = out.paths.root.join(dir.trim_start_matches("/"));

						if path.exists() {
							out.paths.sources.push(path);
						}
						else {
							errs.push(SquarkError::Unrecoverable {
								msg: slash!("a source folder you specified does not exist: {}", path),
								hint: format!("{WHITE}paths.sources{GREEN} folders are relative from your project root"),
								debug: vec![],
							});
						}
					}
				}
			}

			if let Some(values) = Self::get_string_array(&paths, "paths", "include", "(RegEx patterns)", &mut errs) {
				for value in values {
					if let Some(pattern) = Self::require_string_entry(value, "paths", "include", "(RegEx patterns)", &mut errs) {
						match regex::Regex::new(&pattern) {
							Ok(compiled) => out.paths.exclude.push(compiled),
							Err(e)       => errs.push(SquarkError::external(e)),
						}
					}
				}
			}

			if let Some(values) = Self::get_string_array(&paths, "paths", "exclude", "(RegEx patterns)", &mut errs) {
				for value in values {
					if let Some(pattern) = Self::require_string_entry(value, "paths", "exclude", "(RegEx patterns)", &mut errs) {
						match regex::Regex::new(&pattern) {
							Ok(compiled) => out.paths.exclude.push(compiled),
							Err(e)       => errs.push(SquarkError::external(e)),
						}
					}
				}
			}

			match paths.get("default-exclude") {
				Some(toml::Value::Boolean(false)) => { out.paths.exclude.clear(); }
				Some(toml::Value::Boolean(true)) => (),
				Some(v) => errs.push(SquarkError::Recoverable {
					msg: format!("you provided an invalid {YELLOW}paths.default-exclude{RED} of type {}", v.type_str()),
					hint: format!("{WHITE}paths.default-exclude{GREEN} must be a Boolean"),
					debug: vec![format!("you provided {v}")]
				}),
				None => (),
			}
		}

		if errs.is_empty() {
			Ok(out)
		} else {
			Err(SquarkError::Multiple { errs })
		}
	}
}

impl SquarkupConfig
{
	/// Validate that `data.field` is an array.
	fn get_string_array<'d>(
		data: &'d toml::Value,
		category: &'static str,
		field: &'static str,
		hint: &'static str,
		errs: &mut Vec<SquarkError>,
	) -> Option<&'d Vec<toml::Value>>
	{
		match data.get(field)
		{
			Some(toml::Value::Array(values)) => Some(values),
			Some(v) => {
				errs.push(SquarkError::Recoverable {
					msg: format!("invalid setting for {YELLOW}{category}.{field}{RED}"),
					hint: format!("{YELLOW}{category}.{field}{GREEN} must be an array of strings {GREY}{hint}"),
					debug: vec![format!("you provided {GREY_LIGHT}{v}{GREY}, which has type: {GREY_LIGHT}{}{GREY}", v.type_str())],
				});
				None
			},
			None => None,
		}
	}

	/// Validate that `data.field` is a string in an array.
	fn require_string_entry<'d>(
		data: &'d toml::Value,
		category: &'static str,
		field: &'static str,
		hint: &'static str,
		errs: &mut Vec<SquarkError>,
	) -> Option<&'d String>
	{
		match data
		{
			toml::Value::String(value) => Some(value),
			v => {
				errs.push(SquarkError::Recoverable {
					msg: format!("invalid setting for an entry of {YELLOW}{category}.{field}{RED}"),
					hint: format!("{YELLOW}{category}.{field}{GREEN} entries must be strings {GREY}{hint}"),
					debug: vec![format!("you provided {GREY_LIGHT}{v}{GREY}, which has type: {GREY_LIGHT}{}{GREY}", v.type_str())],
				});
				None
			},
		}
	}
}
