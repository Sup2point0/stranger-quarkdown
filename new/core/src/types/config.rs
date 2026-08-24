use std::path::PathBuf;

use enum_stringify::EnumStringify;


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
	pub repo: PathBuf,

	/// The directory containing the user's SvelteKit site.
	pub site: PathBuf,

	/// Where in the site all `dest` fields are relative to.
	pub dest: PathBuf,
	
	pub sources: Vec<String>,
	pub exclude: Vec<String>,
}

#[derive(Clone, Debug)]
pub struct ErrorConfig
{
	/// What to do when an error is encountered.
	pub on_error: ErrorAction,

	/// What to do when a target file to write to already exists.
	pub on_file_exists: FileAction,
}

#[derive(EnumStringify)] #[enum_stringify(case = "flat")]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ErrorAction
{
	/// Log the error, recover and continue.
	WARN,

	/// Crash Squarkdown and exit.
	KILL,
}

#[derive(EnumStringify)] #[enum_stringify(case = "flat")]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FileAction
{
	/// Return an error, handled according to `config.errors.on_error`.
	ERROR,

	/// Skip regenerating this file.
	SKIP,

	/// Overwrite the existing file.
	OVERWRITE,
}
