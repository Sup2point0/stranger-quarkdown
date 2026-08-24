use enum_stringify::EnumStringify;

use crate::{
	SquarkResult, SquarkError,
	utils::colours::*,
	utils::macros::*,
};

use std::path::PathBuf;


/// The user's squarkup configuration, loaded from `.squarkdown/squarkup.json`.
#[derive(Clone, Debug)]
pub struct SquarkupConfig
{
	pub paths: PathsConfig,
	pub errors: ErrorConfig,
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
	
	pub sources: Vec<String>,
	pub exclude: Vec<String>,
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
		Self {
			paths: PathsConfig {
				root: root.clone(),
				site: root.clone(),
				dest: root.join("src/routes/"),
				sources: vec![],
				exclude: vec![],
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

		if let Some(paths) = data.get("paths")
		{
			match paths.get("site") {
				Some(toml::Value::String(dir)) => {
					let path = self.paths.root.join(dir);
					if !path.exists() {
						errs.push(SquarkError::Unrecoverable {
							msg: str!("the directory you specified for your SvelteKit site doesn't exist!"),
							hint: format!("{YELLOW}paths.site{WHITE} is relative from the root directory of your project"),
							debug: vec![format!("{} is not a valid directory", path.display())],
						});
					}
					self.paths.site = path;
				},
				Some(v) => {
					errs.push(SquarkError::Unrecoverable {
						msg: format!("you provided a {} for {YELLOW}paths.site{RED}", v.type_str()),
						hint: format!("{YELLOW}paths.site{WHITE} must be a string"),
						debug: vec![],
					});
				},
				None => todo!(),
			}
		}

		if errs.is_empty() {
			Ok(())
		} else {
			Err(SquarkError::ManyRecoverable { errs })
		}
	}
}
