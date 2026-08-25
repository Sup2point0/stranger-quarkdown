use enum_stringify::EnumStringify;

use crate::{
	SquarkResult, SquarkError,
	colours::*,
	macros::*,
};

use std::path::PathBuf;


/// The user's squarkup configuration, loaded from `.squarkdown/squarkup.json`.
#[derive(Clone, Debug)]
pub struct SquarkupConfig
{
	pub paths:  PathsConfig,
	pub errors: ErrorConfig,
	pub out:    OutConfig,
	pub bases:  BasesConfig,
	pub styles: StylesConfig,
	pub assets: AssetsConfig,
	pub fonts:  FontsConfig,
}

#[derive(Clone, Debug)]
pub struct PathsConfig
{
	/// The root directory of the user's project, from which squarkup begins.
	pub root: PathBuf,

	/// The directory containing the user's SvelteKit site.
	pub site: PathBuf,

	/// Where in the site all `dest` fields are relative to.
	pub dest: PathBuf,
	
	/// Source directories from which to start searching for Markdown files.
	pub sources: Vec<PathBuf>,

	/// Only files whose full path matches against any of these patterns will be squarked up.
	pub include: Vec<String>,
	/// Cache of `.include` compiled to RegEx patterns.
	pub include_patterns: Vec<regex::Regex>,

	/// Files whose full path matches against any of these patterns will *not* be squarked up.
	pub exclude: Vec<String>,
	/// Cache of `.exclude` compiled to RegEx patterns.
	pub exclude_patterns: Vec<regex::Regex>,

	/// Include a default set of sensible exclude patterns, like `.git/` and `node_modules/`?
	pub default_exclude: bool,
}

#[derive(Clone, Debug, Default)]
pub struct ErrorConfig
{
	/// What to do when a non-fatal error is encountered (e.g. parsing a file failed).
	pub on_error: ErrorAction,

	/// What to do when a target file to write to already exists.
	pub on_file_exists: FileAction,
}

#[derive(EnumStringify)] #[enum_stringify(case = "flat")]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum ErrorAction
{
	/// Log the error, recover and continue.
	#[default] WARN,

	/// Crash Squarkdown and exit.
	KILL,
}

#[derive(EnumStringify)] #[enum_stringify(case = "flat")]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum FileAction
{
	/// Overwrite the existing file.
	#[default] OVERWRITE,

	/// Return an error, handled according to `config.errors.on_error`.
	ERROR,

	/// Skip regenerating this file.
	SKIP,
}


impl SquarkupConfig
{
	/// Construct a `SquarkupConfig` with defaults applied and *resolved* against `root`.
	/// 
	/// We can't implement `Default` because paths depend on the project `root`, which is only available at runtime!
	pub fn init_defaults(root: PathBuf) -> Self
	{
		/* NOTE: This is the canonical source of truth for Squarkdown's defaults, make sure to sync docs with this! */
		Self {
			paths: PathsConfig {
				root: root.clone(),
				site: root.clone(),
				dest: root.join("src/routes/"),
				sources: vec![root.clone()],
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
			errors: Default::default(),
		}
	}

	/// Load the settings specified in TOML `data` into the config, and validate their values.
	/// 
	/// Returns `Err(SquarkError::ManyRecoverable)` only if nonzero errors are encountered.
	pub fn set_from_toml(&mut self, data: toml::Table) -> SquarkResult
	{
		let mut errs = vec![];

		// TODO finish implementing fields

		if let Some(paths) = data.get("paths")
		{
			match paths.get("site") {
				Some(toml::Value::String(dir)) => {
					let path = self.paths.root.join(dir.trim_start_matches("/"));

					if path.exists() {
						self.paths.site = path;
						self.paths.dest = self.paths.site.join("src/routes/");
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
				None => todo!(),
			}

			match paths.get("sources") {
				Some(toml::Value::Array(values)) => {
					self.paths.sources.clear();

					for value in values {
						match value {
							toml::Value::String(dir) => {
								let path = self.paths.root.join(dir.trim_start_matches("/"));

								if path.exists() {
									self.paths.sources.push(path);
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

					if self.paths.sources.is_empty() {
						self.paths.sources = vec![self.paths.root.clone()];
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
			Ok(())
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
