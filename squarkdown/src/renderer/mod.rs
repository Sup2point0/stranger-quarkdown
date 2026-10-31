//! This module implements the renderer, which outputs `+page.svx` and `+page.ts` files.

mod svx_renderer;
mod ts_renderer;
use svx_renderer::{ Renderer };

mod ctx;
pub(crate) use ctx::{ RenderCtx };

#[cfg(test)]
mod test_utils;


// == PUBLIC == //

use crate::prelude::*;
use crate::types::Never;

use std::sync::atomic::{ AtomicUsize, Ordering };
use std::fs;


/// Render `page` to its `+page.svx` and/or `+page.ts` files, applying `site` and `config` accordingly.
/// 
/// This always returns `Err(SquarkError)` for more uniform error propagation.
/// 
/// If both `+page.svx` and `+page.ts` (if wanted) were successfully written, `successful_writes` is incremented.
/// 
/// If `dry-run`, do not write to any actual files.
pub fn render(
	page: &PageData,
	site: &SiteData,
	config: &SquarkupConfig,
	dry_run: bool,
	successful_writes: &AtomicUsize,
) -> SquarkResult<Never>
{
	let renderer = Renderer::new(page, site, config);

	if !dry_run {
		fs::create_dir_all(&renderer.dest_folder)?;
	}

	/* NOTE:
		Logically speaking the `+page.ts` renderer should be separate from the `+page.svx` renderer, but I decided with all the duplication required, it's not worth splitting them.

		This means a little rigidity in that `render_page_ts()` (which borrows) must happen before `render_page_svx()` (which moves), but I doubt we're changing the render targets anytime soon, so it's not a problem :]
	*/

	let mut wrote_ts = true;

	if config.out.render_page_ts {
		wrote_ts = false;
		renderer.render_page_ts(&mut wrote_ts, dry_run)?;
	}
	
	let mut wrote_svx = false;
	let ((), errs) = renderer.render_page_svx(&mut wrote_svx, dry_run)?;

	if (wrote_ts && wrote_svx) {
		successful_writes.fetch_add(1, Ordering::Relaxed);
	}

	Err(errs)
}
