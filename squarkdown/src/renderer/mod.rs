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
/// 
/// Returns `(errs, wrote)` where `wrote` indicates if all required files were successfully written.
pub fn render(
	page: &PageData,
	site: &SiteData,
	config: &SquarkupConfig,
	dry_run: bool,
) -> (SquarkResult, bool)
{
	let mut errs = SquarkError::multiple(fmt!(
		"rendering {B}{}",
		utils::display_rel(&page.filepath, &config.paths.root)
	));

	let renderer = Renderer::new(page, site, config);

	if !dry_run {
		let r = fs::create_dir_all(&renderer.dest_folder);

		if let Err(e) = r {
			return (Err(e.into()), false);
		}
	}

	/* NOTE:
		Logically speaking the `+page.ts` renderer should be separate from the `+page.svx` renderer, but I decided with all the duplication required, it's not worth splitting them.

		This means a little rigidity in that `render_page_ts()` (which borrows) must happen before `render_page_svx()` (which moves), but I doubt we're changing the render targets anytime soon, so it's not a problem :]
	*/

	let mut wrote_ts = true;

	if config.out.render_page_ts {
		wrote_ts = false;
		catch!(errs => { renderer.render_page_ts(&mut wrote_ts, dry_run)?; });
	}
	
	let mut wrote_svx = false;

	catch!(errs => { renderer.render_page_svx(&mut wrote_svx, dry_run)?; });

	(errs.or(()), wrote_ts && wrote_svx)
}
