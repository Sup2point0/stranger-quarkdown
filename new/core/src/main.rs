use squarkdown::*;
use squarkdown::core::*;
use squarkdown::colours::*;

use std::fs::File;
use std::process::ExitCode;
use std::time::Instant;


fn main() -> ExitCode
{
	println!();
	println!("{P}Squarkdown v{}", "4.0");
	log::line();

	let t_init = Instant::now();
	let status = squarkup();
	let t = t_init.elapsed();
	log::line();

	match status
	{
		Ok(_) => {
			println!("{P}squarkup finished! {GREY}{:.2?} ms{W}", t.as_secs_f64() * 1000.0);
			ExitCode::SUCCESS
		},
		Err(e) => {
			log::error(e);
			log::line();
			println!("{R}squarkup failed! {GREY}{:.2?} ms\n{W}", t.as_secs_f64() * 1000.0);
			ExitCode::FAILURE
		},
	}
}

/// Run squarkup on the user's project.
/// 
/// Returns:
/// 
/// - `Ok(true)` if squarkup was attempted and was successful
/// - `Err(msg)` if squarkup was attempted but failed
/// - `Ok(false)` if no squarkup was attempted
fn squarkup() -> SquarkResult
{
	/* NOTE: We're intentionally keeping the main pipeline under one scope so all the shared variables are easily accessible instead of requiring a whole load of messy parameter-passing. Some loss in readability, but gains in concision ;) */

	// == SETUP == //
	let project_root = resolver::resolve_project_root()?;
	log::ok!(slash!("found your project: {B}{}", project_root));

	let config = resolver::resolve_config(&project_root)?;
	log::ok!(slash!("found your site: {B}{}", config.paths.site));

	let mut site_data = SiteData::new();
	
	// == PARSE == //
	log::is!("finding files to squarkup...");

	let mut errs = SquarkError::multiple();
	let mut tried = 0;
	
	for filepath in resolver::resolve_files(&config) {
		tried += 1;

		catch!(errs => {
			let filepath = filepath?;
			let r = parser::parse(&filepath, &config)?;
			
			if let Some(page) = r {
				site_data.add_page(page);
				
				log::info!(slash!(
					"found active file: {}/{GREY1}{}",
					filepath.parent().unwrap().strip_prefix(&config.paths.root).unwrap().to_path_buf(),
					filepath.file_name().unwrap().to_string_lossy(),
				));
			}
		});
	}

	errs.depends(&config)?;
	
	if tried == 0 {
		return Err(SquarkError::Unrecoverable {
			msg: str!("no files found to squarkup"),
			hint: fmt!("check your {W}paths.sources{G}, {W}paths.include{G}, {W}paths.exclude{G} are configured correctly?"),
			debug: vec![],
		});
	}
	else if site_data.stats.active_pages == 0 {
		return Err(SquarkError::Unrecoverable {
			msg: str!("no active files found"),
			hint: fmt!("check your files have {W}<!-- #SQUARK live!{G} under their heading"),
			debug: vec![
				fmt!("parsed {tried} files"),
			],
		});
	}
	else {
		log::ok!("found {} active files to squarkup", site_data.stats.active_pages);
	}

	if config.errors.strict {
		let r = site_data.check_conflicts(&config);
		
		if let Err(e) = r {
			e.depends(&config)?;
		}
	}

	// == RENDER == //
	log::is!("rendering...");

	for page in site_data.pages() {
		let r = renderer::render(page, &site_data, &config);

		if let Err(e) = r {
			e.depends(&config)?;
		}
	}

	// == SITE DATA == //
	if let Some(ref dest) = config.out.site_data_path {
		log::is!("saving site data...");

		let data_raw = site_data.serialise(&config);
		let file = File::create(dest)?;
		serde_json::to_writer_pretty(file, &data_raw).map_err(err!())?;

		log::ok!(slash!("saved site data to {B}{}", dest.to_path_buf()));
	}
	
	Ok(())
}
