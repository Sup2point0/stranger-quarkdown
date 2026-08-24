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
	pub fn try_from_toml(data: toml::Table, root: PathBuf) -> SquarkResult<Self>
	{
		// let site_raw = (|| {
		// 	let paths = data.get("paths")?;
		// 	let raw = paths.get("site")?;
		// 	let site = raw.as_str()?;
		// 	let full = root.join(site.to_string())
		// 	Some(full)
		// })().unwrap_or(root);

		let mut errs = vec![];

		let out = Self {
			paths: PathsConfig {
				root: root.clone(),
				site: match &data["paths"]["site"] {
					toml::Value::String(str) => root.clone().join(str),
					v => {
						errs.push(SquarkError::Recoverable {
							msg: format!("{WHITE}paths.site{RED} must be a string, but you gave: {v}"),
						});
						root.clone()
					},
				},
				dest: PathBuf::new(),
				sources: vec![],
				exclude: vec![],
			},
			errors: Default::default(),
		};

		if errs.is_empty() {
			Ok(out)
		} else {
			Err(SquarkError::ManyRecoverable { errors: errs })
		}
	}
}
