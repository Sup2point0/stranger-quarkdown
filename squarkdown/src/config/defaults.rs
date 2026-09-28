use super::*;
use crate::macros::*;

use path_macro::path;
use regex::regex;

use std::path::{ Path, PathBuf };


impl SquarkupConfig
{
	/// Construct a `SquarkupConfig` with defaults applied and *resolved* against `root`.
	/// 
	/// We can't implement `Default` because paths depend on the project `root`, which is only available at runtime!
	#[must_use]
	pub fn init_defaults(root: &Path, site: &Path) -> Self
	{
		/* NOTE: This is the canonical source of truth for Squarkdown's defaults, make sure to sync docs with this! */
		Self {
			paths: PathsConfig {
				root: root.to_owned(),
				site: site.to_owned(),
				sources: vec![PathBuf::new()],
				include: vec![
					regex!(r"\.md$").clone(),
				],
				exclude: vec![
					regex!(r"/\.git/").clone(),
					regex!(r"/node_modules/").clone(),
					regex!(r"/.svelte-kit/").clone(),
				],
			},
			out: OutConfig {
				folder: path!(site / "src/routes/"),
				file_name: str!("+page.svx"),
				render_page_ts: true,
				shorter_fields: false,
				site_data_path: None,
			},
			format: FormatConfig {
				preserve_heading: false,
				preserve_comments: false,
				externalise_links: false,
				mark_invalid_links: false,
			},
			styles: StylesConfig { folder: None, base_file: None },
			assets: AssetsConfig { folder: root.to_owned(), site_assets_folder: None,
				extensions: vec![
					str!("png"), str!("jpg"), str!("jpeg"), str!("webp"), str!("svg"),
				],
			},
			fonts:  FontsConfig { queries: vec![] },
			errors: ErrorConfig {
				strict: true,
				on_error: ErrorAction::WARN,
				file_already_exists: FileAction::OVERWRITE,
				inactive_link: LinkRewriteAction::STRIP_EXTENSION,
			},
		}
	}
}

