use crate::resolver;
use crate::prelude::*;
use crate::log;
use crate::colours::*;
use crate::macros::*;


pub fn prep_assets(site_data: &mut SiteData, config: &SquarkupConfig) -> SquarkResult
{
	let mut errs = SquarkError::multiple("preprocessing assets");

	for paths in resolver::resolve_assets(&config) {
		catch!(errs => {
			let (source_path, dest_path) = paths?;

			log::info!(slash!(
				"found asset: {GREY1}{}",
				source_path.strip_prefix(&config.paths.root).unwrap()
			));

			if let Some(parent) = dest_path.parent() {
				std::fs::create_dir_all(parent)?;
			}

			std::fs::copy(&source_path, &dest_path)?;

			site_data.stats.assets += 1;
		});
	}

	if site_data.stats.assets == 0 {
		errs.push(SquarkError::Recoverable {
			msg: str!("no assets found to copy"),
			hint: fmt!("check your {Y}assets.folder{G}, {Y}assets.site-assets-folder{G}, {Y}assets.extensions{G} are configured correctly?"),
			debug: vec![],
		});
	}

	errs.or(())
}
