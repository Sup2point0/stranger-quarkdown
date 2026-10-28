use crate::resolver;
use crate::prelude::*;
use crate::utils;
use crate::utils::log;
use crate::utils::colours::*;
use crate::utils::macros::*;


pub fn prep_assets(
	site_data: &mut SiteData,
	config: &SquarkupConfig,
	dry_run: bool,
) -> SquarkResult
{
	let mut errs = SquarkError::multiple("preprocessing assets");

	for paths in resolver::resolve_assets(config) {
		catch!(errs => {
			let (source_path, dest_path) = paths?;

			let (folder, filename) = utils::split_path(&source_path);

			log::info!(
				"found asset: {folder}{FF}{filename}",
			);

			if !dry_run {
				if let Some(parent) = dest_path.parent() {
					std::fs::create_dir_all(parent)?;
				}
				std::fs::copy(&source_path, &dest_path)?;
			}

			site_data.stats.assets += 1;
		});
	}

	if site_data.stats.assets == 0 {
		errs.push(SquarkError::Recoverable {
			msg: str!("no assets found to copy"),
			hint: fmt!("check your assets{Y}.folder{G} · {Y}.site-assets-folder{G} · {Y}.extensions{G} are configured correctly?"),
			debug: vec![],
		});
	}

	errs.or(())
}
