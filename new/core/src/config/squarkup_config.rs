use enum_stringify::EnumStringify;

use crate::{
	macros::*,
};

use std::path::PathBuf;


/// The user's complete squarkup configuration, loaded from `.squarkdown/squarkup.json`.
#[derive(Clone, Debug)]
pub struct SquarkupConfig
{
	pub paths:  PathsConfig,
	pub out:    OutConfig,
	pub data:   DataConfig,
	pub bases:  BasesConfig,
	pub styles: StylesConfig,
	pub assets: AssetsConfig,
	pub fonts:  FontsConfig,
	pub errors: ErrorConfig,
}


#[derive(Clone, Debug)]
pub struct PathsConfig {
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


#[derive(Clone, Debug)]
pub struct OutConfig {
	// TODO migrate dest to this
	folder: PathBuf,

	/// The file name for exported files, including the (expected) `.svx` extension.
	file: String,
}


#[derive(Clone, Debug)]
pub struct DataConfig {
	/// Where to export site data, including the (expected) `.json` extension.
	path: PathBuf,
}


#[derive(Clone, Debug)]
pub struct BasesConfig {
	folder: PathBuf,
	page_js: PathBuf,
}


#[derive(Clone, Debug)]
pub struct StylesConfig {
	folder: PathBuf,
	base_file: PathBuf,
}


#[derive(Clone, Debug)]
pub struct AssetsConfig {
	folder: PathBuf,
	site_assets_folder: PathBuf,
	extensions: Vec<String>,
}


#[derive(Clone, Debug)]
pub struct FontsConfig {
	queries: Vec<String>,
}


#[derive(Clone, Debug, Default)]
pub struct ErrorConfig {
	/// What to do when a non-fatal error is encountered (e.g. parsing a file failed).
	pub on_error: ErrorAction,

	/// What to do when a target file to write to already exists.
	pub on_file_exists: FileAction,
}

#[derive(EnumStringify)] #[enum_stringify(case = "flat")]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum ErrorAction {
	/// Log the error, recover and continue.
	#[default] WARN,

	/// Crash Squarkdown and exit.
	KILL,
}

#[derive(EnumStringify)] #[enum_stringify(case = "flat")]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum FileAction {
	/// Overwrite the existing file.
	#[default] OVERWRITE,

	/// Return an error, handled according to `config.errors.on_error`.
	ERROR,

	/// Skip regenerating this file.
	SKIP,
}
