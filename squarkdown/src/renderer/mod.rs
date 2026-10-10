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
		fmt!(
			"rendering {B}{}",
			utils::display_rel(&page.filepath, &config.paths.root)
		)
	);

	let renderer = Renderer::new(page, site, config);
	fs::create_dir_all(&renderer.dest_folder)?;

	/* NOTE:
		Logically speaking the `+page.ts` renderer should be separate from the `+page.svx` renderer, but I decided with all the duplication required, it's not worth splitting them.

		This means a little rigidity in that `render_page_ts()` (which borrows) must happen before `render_page_svx()` (which moves), but I doubt we're changing this, so it's not a problem :]
	*/

	if config.out.render_page_ts {
		catch!(errs => { renderer.render_page_ts()?; });
	}
	catch!(errs => { renderer.render_page_svx()?; });

	errs.or(())
}
