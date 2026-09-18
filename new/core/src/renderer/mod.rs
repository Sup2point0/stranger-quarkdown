mod svx_renderer; use svx_renderer::{ Renderer };
mod ts_renderer;
mod ctx; pub use ctx::{ RenderCtx };

#[cfg(test)] mod test_utils;


use crate::core::*;
use crate::macros::*;

use std::fs;


/// Render `page` to its `+page.svx` and/or `+page.ts` files, applying `site` and `config` accordingly.
pub fn render(
	page: &PageData,
	site: &SiteData,
	config: &SquarkupConfig,
) -> SquarkResult
{
	let mut errs = SquarkError::multiple();
	let mut renderer = Renderer::new(page, site, config);

	if !renderer.dest_folder.exists() {
		fs::create_dir_all(&renderer.dest_folder)?;
	}

	catch!(errs => { renderer.render_page_ts()?; });
	catch!(errs => { renderer.render_page_svx()?; });

	errs.or(())
}
