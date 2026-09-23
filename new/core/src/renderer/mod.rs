//! This module implements the renderer, which outputs `+page.svx` and `+page.ts` files.

mod svx_renderer;
use svx_renderer::{ Renderer };

mod ts_renderer;

mod ctx;
pub(crate) use ctx::{ RenderCtx };

#[cfg(test)]
mod test_utils;


// == PUBLIC == //

use crate::prelude::*;
use crate::utils;
use crate::colours::*;
use crate::macros::*;

use std::fs;


/// Render `page` to its `+page.svx` and/or `+page.ts` files, applying `site` and `config` accordingly.
pub fn render(
	page: &PageData,
	site: &SiteData,
	config: &SquarkupConfig,
) -> SquarkResult
{
	let mut errs = SquarkError::multiple(
		fmt!("rendering {B}{}", utils::display_rel(&page.filepath, &config.paths.root))
	);

	let renderer = Renderer::new(page, site, config);

	if !renderer.dest_folder.exists() {
		fs::create_dir_all(&renderer.dest_folder)?;
	}

	catch!(errs => { renderer.render_page_ts()?; });
	catch!(errs => { renderer.render_page_svx()?; });

	errs.or(())
}
