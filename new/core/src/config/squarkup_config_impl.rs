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
			match paths.get("sources") {
				Some(toml::Value::Array(values)) => for value in values {
					match value {
						toml::Value::String(dir) => {
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
						},
						v => errs.push(SquarkError::Unrecoverable {
							msg: format!("you provided an invalid {YELLOW}paths.sources{RED} entry of type {}", v.type_str()),
							hint: format!("{WHITE}paths.sources{GREEN} entries must be strings {GREY}(filepaths relative to your project root)"),
							debug: vec![],
						}),
					}
				},
				Some(v) => errs.push(SquarkError::Unrecoverable {
					msg: format!("you provided an invalid {YELLOW}paths.sources{RED} of type {}", v.type_str()),
					hint: format!("{WHITE}paths.sources{GREEN} must be an array of strings {GREY}(filepaths relative to your project root)"),
					debug: vec![],
				}),
				None => (),
			}

			match paths.get("include") {
				Some(toml::Value::Array(values)) => for value in values {
					match value {
						toml::Value::String(pattern) => {
							match regex::Regex::new(&pattern) {
								Ok(compiled) => out.paths.include.push(compiled),
								Err(e)       => errs.push(SquarkError::external(e)),
							}
						},
						v => errs.push(SquarkError::Unrecoverable {
							msg: format!("you provided an invalid {YELLOW}paths.include{RED} entry of type {}", v.type_str()),
							hint: format!("{WHITE}paths.include{GREEN} entries must be strings {GREY}(RegEx patterns)"),
							debug: vec![format!("{values:?}")],
						}),
					}
				},
				Some(v) => errs.push(SquarkError::Unrecoverable {
					msg: format!("you provided an invalid {YELLOW}paths.include{RED} of type {}", v.type_str()),
					hint: format!("{WHITE}paths.include{GREEN} must be an array of strings {GREY}(RegEx patterns)"),
					debug: vec![],
				}),
				None => (),
			}

			match paths.get("exclude") {
				Some(toml::Value::Array(values)) => for value in values {
					match value {
						toml::Value::String(pattern) => {
							match regex::Regex::new(&pattern) {
								Ok(compiled) => out.paths.exclude.push(compiled),
								Err(e)       => errs.push(SquarkError::external(e)),
							}
						},
						v => errs.push(SquarkError::Recoverable {
							msg: format!("you provided an invalid {YELLOW}paths.exclude{RED} entry of type {}", v.type_str()),
							hint: format!("{WHITE}paths.exclude{GREEN} entries must be strings {GREY}(RegEx patterns)"),
							debug: vec![format!("{values:?}")],
						}),
					}
				},
				Some(v) => errs.push(SquarkError::Recoverable {
					msg: format!("you provided an invalid {YELLOW}paths.exclude{RED} of type {}", v.type_str()),
					hint: format!("{WHITE}paths.exclude{GREEN} must be an array of strings {GREY}(RegEx patterns)"),
					debug: vec![],
				}),
				None => (),
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
