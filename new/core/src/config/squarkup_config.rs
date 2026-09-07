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
	pub format: FormatConfig,
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
	
	/// Source directories from which to start searching for Markdown files.
	pub sources: Vec<PathBuf>,

	/// Only files whose full path matches against any of these RegEx patterns will be squarked up.
	pub include: Vec<regex::Regex>,

	/// Files whose full path matches against any of these RegEx patterns will *not* be squarked up.
	pub exclude: Vec<regex::Regex>,
}

#[derive(Clone, Debug)]
pub struct OutConfig {
	/// Where to export files relative to, relative to `.paths.site`.
	/// 
	/// `dest` paths in files are relative to this folder.
	pub folder: PathBuf,

	/// The file name for exported files, including the (expected) `.svx` extension.
	pub file: String,
}

#[derive(Clone, Debug)]
pub struct DataConfig {
	/// Where to export site data, including the (expected) `.json` extension.
	pub path: PathBuf,
}

#[derive(Clone, Debug)]
pub struct FormatConfig {
	/// Strip `<!-- comments -->` from the rendered output?
	pub preserve_comments: bool,

	/// Convert links containing `<sup>↗</sup>` to `<a target="_blank">` elements?
	pub externalise_links: bool,
}

#[derive(Clone, Debug)]
pub struct BasesConfig {
	pub folder: Option<PathBuf>,
	pub page_js: Option<PathBuf>,
}

#[derive(Clone, Debug)]
pub struct StylesConfig {
	pub folder: Option<PathBuf>,
	pub base_file: Option<PathBuf>,
}

#[derive(Clone, Debug)]
pub struct AssetsConfig {
	pub folder: Option<PathBuf>,
	pub site_assets_folder: Option<PathBuf>,
	pub extensions: Vec<String>,
}

#[derive(Clone, Debug)]
pub struct FontsConfig {
	pub queries: Vec<String>,
}


#[derive(Clone, Debug)]
pub struct ErrorConfig {
	/// What to do when a non-fatal error is encountered (e.g. parsing a file failed).
	pub on_error: ErrorAction,

	/// What to do when a target file to write to already exists.
	pub on_file_exists: FileAction,
}

#[derive(EnumStringify)] #[enum_stringify(case = "flat")]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ErrorAction {
	/// Log the error, recover and continue.
	WARN,

	/// Crash Squarkdown and exit.
	KILL,
}

#[derive(EnumStringify)] #[enum_stringify(case = "flat")]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FileAction {
	/// Overwrite the existing file.
	OVERWRITE,

	/// Return an error, handled according to `config.errors.on_error`.
	ERROR,

	/// Skip regenerating this file.
	SKIP,
}
