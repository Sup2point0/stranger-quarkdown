use super::*;
use crate::*;
use crate::prelude::*;
use crate::utils::log;
use crate::utils::colours::*;

use rayon::prelude::*;

use std::process::ExitCode;
use std::time::Instant;


/// Run Squarkdown on the user's project.
/// 
/// This includes squarkup as well as extras like fonts and assets preprocessing.
pub fn squarkdown(args: impl Iterator<Item = String>) -> ExitCode
{
	let mut dry_run = false;
	let mut assets  = false;
	let mut fonts   = false;
	let mut unknown_args = vec![];

	for arg in args {
		if      arg.eq_ignore_ascii_case("--quiet")   { let _ = QUIET.set(()); }
		else if arg.eq_ignore_ascii_case("--dry-run") { dry_run = true; }
		else if arg.eq_ignore_ascii_case("--assets")  { assets = true; }
		else if arg.eq_ignore_ascii_case("--fonts")   { fonts = true; }
		else {
			unknown_args.push(arg);
		}
	}

	if !unknown_args.is_empty() {
		shared::unknown_args(&unknown_args);
		return ExitCode::FAILURE
	}

	log::version();

	let t_init = Instant::now();
	let r = squarkup(dry_run, assets, fonts);
	let t = t_init.elapsed();
	let perf = t.as_secs_f64() * 1000.0;

	match r
	{
		Ok(..) => {
			log::line();
			println!("{P}squarkup finished! {GREY}{perf:.1?} ms{W}");
			return ExitCode::SUCCESS
		},
		Err(e) => {
			log::error(e);
			println!("{R}squarkup failed! {GREY}{perf:.1?} ms\n{W}");
			return ExitCode::FAILURE
		},
	}
}

/// The core squarkup pipeline that is timed for performance.
fn squarkup(
	dry_run: bool,
	assets: bool,
	fonts: bool,
) -> SquarkResult
{
	/* NOTE: We're intentionally keeping the main pipeline under one scope so all the shared variables are easily accessible instead of requiring a whole load of messy parameter-passing. Some loss in readability, but gains in concision ;) */

	if dry_run || assets || fonts {
		let mut enabled = vec![];
		if dry_run { enabled.push("dry-run"); }
		if assets  { enabled.push("assets"); }
		if fonts   { enabled.push("fonts"); }
		log::info!("enabled options: {G}{}", enabled.join(&fmt!("{GREY} · {G}")));
	}

	// == SETUP == //
	let project_root = resolver::resolve_project_root()?;
	log::ok!(slash!("found your project: {B}{}", project_root));

	let config = resolver::resolve_config(&project_root)?;
	log::ok!(slash!("found your site: {B}{}", config.paths.site));

	let mut site_data = SiteData::new();

	// == FONTS == //
	if fonts {
		log::is!("injecting fonts queries...");

		extras::prep_fonts(&config, dry_run)
			.or_else(|e| e.depends(&config))?;

		log::ok!("injected fonts queries");
	}

	// == ASSETS == //
	if assets {
		log::is!("preprocessing assets...");

		extras::prep_assets(&mut site_data, &config, dry_run)
			.or_else(|e| e.depends(&config))?;
	}

	// == PARSE == //
	log::is!("finding files to squarkup...");
	
	for filepath in resolver::resolve_files(&config) {
		site_data.stats.checked_files += 1;

		catch! {
			let filepath = filepath?;
			let r = parser::parse(&filepath, &config)?;
			
			if let Some(page) = r {
				site_data.add_page(page);

				let (folder, file) = utils::split_path_from(filepath, &config.paths.root);
				log::info!("found active file: {folder}/{GREY1}{file}");
			}
		}.or_else(|e| e.depends(&config))?;
	}
	
	if site_data.stats.checked_files == 0 {
		SquarkError::Recoverable {
			msg: str!("no files found to squarkup"),
			hint: fmt!("check your {Y}paths.sources{G} · {Y}.include{G} · {Y}.exclude{G} are configured correctly?"),
			debug: vec![],
		}.depends(&config)?;
	}
	else if site_data.stats.active_pages == 0 {
		SquarkError::Recoverable {
			msg: str!("no active files found"),
			hint: fmt!("check your files have {Y}<!-- #SQUARK live!{G} under their heading"),
			debug: vec![
				fmt!("parsed {} files", site_data.stats.checked_files),
			],
		}.depends(&config)?;
	}
	else {
		log::ok!("found {} active files to squarkup", site_data.stats.active_pages);

		// == CHECK == //
		if config.errors.strict {
			site_data.check_conflicts(&config)
				.or_else(|e| e.depends(&config))?;
		}

		// == RENDER == //
		log::is!(if dry_run {"simulating rendering..."} else {"rendering..."});

		let (errs, num_oks) =
			site_data
			.pages_map()
			.par_iter()
			.map(|(_shard, page)|
				renderer::render(page, &site_data, &config, dry_run)
			)
			.fold(
				|| (SquarkError::multiple(""), 0),
				|(mut errs, mut count), (r, wrote)| {
					if let Err(e) = r {
						let did_push = errs.push(e);
						debug_assert!(did_push);
					}
					if wrote {
						count += 1;
					}
					return (errs, count)
				}
			)
			.reduce(
				|| (SquarkError::multiple(""), 0),
				|(mut e1, c1), (e2, c2)| {
					let did_push = e1.push(e2);
					debug_assert!(did_push);
					return (e1, c1 + c2)
				}
			)
		;

		errs.depends(&config)?;

		log::ok!(
			"{} {num_oks}/{total} pages",
			if dry_run {"would render"} else {"rendered"},
			total = site_data.stats.active_pages
		);
	}

	// == SITE DATA == //
	if let Some(dest) = &config.out.site_data_path {
		log::is!("saving site data...");
		extras::save_site_data(dest, &site_data, &config, dry_run)?;
	}
	
	Ok(())
}
