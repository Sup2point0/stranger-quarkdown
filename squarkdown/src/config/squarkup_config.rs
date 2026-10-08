use crate::macros::*;

use enum_stringify::EnumStringify;

use std::path::PathBuf;


/// The user's complete squarkup configuration.
/// 
/// This is loaded from either a `squarkup.toml` or `squarkup.json` file.
#[derive(Clone, Debug)]
pub struct SquarkupConfig
{
	pub repo: RepoConfig,

	pub paths: PathsConfig,

	/// Options for output.
	pub out: OutConfig,

	/// Options for Markdown rendering.
	pub format: FormatConfig,

	/// Options for asset copying.
	pub assets: AssetsConfig,

	/// Options for Google Fonts query injection.
	pub fonts: FontsConfig,

	/// Error handling strategies.
	pub errors: ErrorConfig,
}


#[derive(Clone, Debug)]
pub struct RepoConfig {
	/// The displayed name of the repo, injected into `<title>` when `format.inject-head = true`.
	pub name: Option<String>,

	/// The GitHub repository name, e.g. `Sup2point0/stranger-quarkdown`.
	pub github: Option<String>,
}

#[derive(Clone, Debug)]
pub struct PathsConfig {
	/// The root directory of the user's project, from which squarkup begins.
	pub root: PathBuf,

	/// The folder containing the user's SvelteKit site.
	pub site: PathBuf,
	
	/// Source folders from which to start searching for Markdown files.
	/// 
	/// `/` is a special entry, treated as 'root-only'; it won’t recurse into any directories. Use this to pick up files like `README.md`, `CHANGELOG.md`, etc.
	pub sources: Vec<PathBuf>,

	/// Only files whose full path matches against any of these RegEx patterns will be searched.
	pub include: Vec<regex::Regex>,

	/// Files whose full path matches against any of these RegEx patterns will *not* be checked by Squarkdown.
	pub exclude: Vec<regex::Regex>,
}

#[derive(Clone, Debug)]
pub struct OutConfig {
	/// Where to export files relative to.
	/// 
	/// `destination` paths in files are relative to this folder.
	pub folder: PathBuf,

	/// The file name for exported files, including the (expected) `.svx` extension.
	pub file_name: String,

	/// Where to export site data, including the (expected) `.json` extension.
	/// 
	/// If absent, site data is not exported.
	pub site_data_path: Option<PathBuf>,

	/// Should `+page.ts` files be exported?
	pub render_page_ts: bool,

	/// Should fields in exported `+page.ts` and `site.json` use more compact identifiers?
	/// 
	/// For instance, `description` is shortened to `desc`, and `last_update` is shortened to `update`.
	pub shorter_fields: bool,
}

#[derive(Clone, Debug)]
pub struct FormatConfig {
	/// Inject a `<head>` element containing `<title>` and `<meta name="description">`?
	pub inject_head: bool,

	/// Keep the first heading in the rendered output
	pub preserve_heading: bool,

	/// Keep `<!-- comments -->` in the rendered output?
	pub preserve_comments: bool,

	/// Convert links containing `<sup>↗</sup>` to `<a target="_blank">` elements?
	pub externalise_links: bool,
}

#[derive(Clone, Debug)]
pub struct AssetsConfig {
	pub folder: PathBuf,
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
	/// - Requiring [`PageData.destination`](crate::types::PageData::destination) to be explicitly provided
	/// - Checking directories remain under your project root
	/// - Checking multiple files don't export to the same directory
	pub strict: bool,

	/// What to do when a non-fatal error is encountered (e.g. parsing a file failed).
	pub on_error: ErrorAction,

	/// What to do when a target file to write to already exists.
	pub file_already_exists: FileAction,

	/// When rendering Markdown, how should Squarkdown handle a link that points to an inactive file?
	pub link_broken: LinkRewriteAction,
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
	/// Attach `class="invalid-link"` to the link element.
	MARK_INVALID,

	/// Still strip the `.md` file extension, but don't do anything else.
	STRIP_EXTENSION,

	/// Replace the link with an absolute link to the original file in the GitHub repo.
	LINK_TO_GITHUB,

	/// Throw an error, handled according to `config.errors.on_error`.
	ERROR,
}
