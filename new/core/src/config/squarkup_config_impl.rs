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
					str!(r"\.md$"),
					str!(r"\.svx$"),
				],
				include_patterns: vec![],
				exclude: vec![
					str!(r"/\.git/"),
					str!(r"/node_modules/"),
					str!(r"/.svelte-kit/"),
				],
				exclude_patterns: vec![],
				default_exclude: true,
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

	/// Load the settings specified in TOML `data` into the config, and validate their values.
	/// 
	/// Returns `Err(SquarkError::ManyRecoverable)` only if nonzero errors are encountered.
	pub fn try_from_toml(data: toml::Table, root: &Path) -> SquarkResult<Self>
	{
		let mut errs = vec![];

		// TODO finish implementing fields

		let mut site = None;
		
		if let Some(paths) = data.get("paths")
		{
			match paths.get("site") {
				Some(toml::Value::String(dir)) => {
					let path = root.join(dir.trim_start_matches("/"));

					if path.exists() {
						site = Some(path);
					}
					else {
						errs.push(SquarkError::Unrecoverable {
							msg: str!("the directory you specified for your SvelteKit site doesn't exist!"),
							hint: format!("{WHITE}paths.site{GREEN} is relative from the root directory of your project"),
							debug: vec![slash!("{} is not a valid directory", path)],
						});
					}
				},
				Some(v) => errs.push(SquarkError::Unrecoverable {
					msg: format!("you provided an invalid {YELLOW}paths.site{RED} of type {}", v.type_str()),
					hint: format!("{WHITE}paths.site{GREEN} must be a string (a filepath relative to root)"),
					debug: vec![],
				}),
				None => (),
			}
		}

		let mut out = Self::init_defaults(root, &site.unwrap_or_else(|| root.to_path_buf()));

		if let Some(paths) = data.get("paths")
		{
			match paths.get("sources") {
				Some(toml::Value::Array(values)) => {
					for value in values {
						match value {
							toml::Value::String(dir) => {
								let path = out.paths.root.join(dir.trim_start_matches("/"));

								if path.exists() {
									out.paths.sources.push(path);
								}
								else {
									errs.push(SquarkError::Unrecoverable {
										msg: slash!("a source directory you specified does not exist: {}", path),
										hint: format!("{WHITE}paths.sources{GREEN} are relative from your project root"),
										debug: vec![],
									});
								}
							},
							v => errs.push(SquarkError::Unrecoverable {
								msg: format!("you provided an invalid {YELLOW}paths.sources{RED} entry of type {}", v.type_str()),
								hint: format!("{WHITE}paths.sources{GREEN} entries must be strings (filepaths relative to your project root)"),
								debug: vec![],
							}),
						}
					}
				},
				Some(v) => errs.push(SquarkError::Unrecoverable {
					msg: format!("you provided an invalid {YELLOW}paths.sources{RED} of type {}", v.type_str()),
					hint: format!("{WHITE}paths.sources{GREEN} must be an array of strings (filepaths relative to your project root)"),
					debug: vec![],
				}),
				None => (),
			}
		}

		if errs.is_empty() {
			Ok(out)
		} else {
			Err(SquarkError::ManyRecoverable { errs })
		}
	}

	/// Compile the user's `.include` and `.exclude` entries into RegEx patterns and cache them to `.(in|ex)clude_patterns`.
	/// 
	/// Fails with `SquarkError::ManyRecoverable` if nonzero patterns fail to compile.
	pub fn compile_patterns(&mut self) -> SquarkResult
	{
		let mut errs = vec![];

		if !self.paths.include.is_empty() {
			for pattern in &self.paths.include {
				match regex::Regex::new(&pattern) {
					Ok(compiled) => self.paths.include_patterns.push(compiled),
					Err(e)       => errs.push(SquarkError::external(e)),
				}
			}
		}
		
		if !self.paths.exclude.is_empty() {
			for pattern in &self.paths.exclude {
				match regex::Regex::new(&pattern) {
					Ok(compiled) => self.paths.exclude_patterns.push(compiled),
					Err(e)       => errs.push(SquarkError::external(e)),
				}
			}
		}

		if errs.is_empty() {
			Ok(())
		} else {
			Err(SquarkError::ManyRecoverable { errs })
		}
	}
}
