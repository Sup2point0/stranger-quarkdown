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
	pub out: OutConfig,

	/// Rendering customisations.
	pub format: FormatConfig,

	pub styles: StylesConfig,
	pub assets: AssetsConfig,
	pub fonts: FontsConfig,

	/// Error handling strategies.
	pub errors: ErrorConfig,
}


#[derive(Clone, Debug)]
pub struct PathsConfig {
	/// The root directory of the user's project, from which squarkup begins.
	pub root: PathBuf,

	/// The directory containing the user's SvelteKit site.
	pub site: PathBuf,
	
	/// Source directories from which to start searching for Markdown files.
	/// 
	/// `/` is a special entry, treated as 'root-only'; it won’t recurse into any directories. Use this to pick up files like `README.md`, `CHANGELOG.md`, etc.
	pub sources: Vec<PathBuf>,

	/// Only directories whose full path matches against any of these RegEx patterns will be searched.
	pub include: Vec<regex::Regex>,

	/// Directories whose full path matches against any of these RegEx patterns will *not* be searched.
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

	/// Should `+page.ts` files be exported?
	pub page_ts: bool,

	/// Where to export site data, including the (expected) `.json` extension.
	/// 
	/// If absent, site data is not exported.
	pub data: Option<PathBuf>,
}

#[derive(Clone, Debug)]
pub struct FormatConfig {
	/// Keep the first heading in the rendered output
	pub preserve_heading: bool,

	/// Keep `<!-- comments -->` in the rendered output?
	pub preserve_comments: bool,

	/// Convert links containing `<sup>↗</sup>` to `<a target="_blank">` elements?
	pub externalise_links: bool,

	/// Mark hyperlinks to nonexistent pages?
	pub mark_invalid_links: bool,
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
	/// Enable stricter checks for safety?
	/// 
	/// This includes:
	/// 
	/// - Checking directories remain inside your project
	/// - Checking multiple files don't export to the same directory
	pub strict: bool,

	/// What to do when a non-fatal error is encountered (e.g. parsing a file failed).
	pub on_error: ErrorAction,

	/// What to do when a target file to write to already exists.
	pub file_already_exists: FileAction,

	/// When rendering Markdown, how should Squarkdown handle a link that points to an inactive file?
	pub inactive_link: LinkRewriteAction,
}

#[derive(EnumStringify)] #[enum_stringify(case = "kebab")]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ErrorAction {
	/// Log the error, recover and continue. For when you don't want one bad file to bring down the whole build.
	WARN,

	/// Crash Squarkdown and exit. Any error is deadly!
	KILL,
}

#[derive(EnumStringify)] #[enum_stringify(case = "kebab")]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FileAction {
	/// Overwrite the existing file.
	OVERWRITE,

	/// Return an error, handled according to `config.errors.on_error`.
	ERROR,

	/// Skip regenerating this file.
	SKIP,
}

#[derive(EnumStringify)] #[enum_stringify(case = "kebab")]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LinkRewriteAction {
	/// Still strip the `.md` file extension, but don't do anything else
	STRIP_EXTENSION,

	/// Replace the link with an absolute link to the original file in the GitHub repo.
	LINK_TO_GITHUB,

	/// Throw an error, handled according to `config.errors.on_error`.
	ERROR,
}
