use squarkdown::*;
use squarkdown::core::*;
use squarkdown::config::*;
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
			print_error(e);
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
fn squarkup() -> SquarkResult<bool>
{
	/* NOTE: We're intentionally keeping the main pipeline under one scope so all the shared variables are easily accessible instead of requiring a whole load of messy parameter-passing. Some loss in readability, but gains in concision ;) */

	// == SETUP == //
	let project_root = resolver::resolve_project_root()?;
	log::ok!(slash!("found your project: {B}{}", project_root));

	let config = resolver::resolve_config(project_root)?;
	log::ok!(slash!("found your site: {B}{}", config.paths.site));

	let mut site_data = SiteData::new();
	log::is!("finding files to squarkup...");

	// == PARSE == //
	let mut errs = SquarkError::multiple();
	
	for filepath in resolver::resolve_files(&config) {
		catch!(errs => {
			let filepath = filepath?;

			let file = File::open(&filepath).map_err(err!())?;
			let mut parser = CharmParser::init(file, filepath.clone()).map_err(err!())?;
			
			if let Some(page) = parser.parse(&config).unwrap() {
				site_data.add_page(page);
				
				log::info!(slash!(
					"found active file: {GREY1}{}",
					filepath.strip_prefix(&config.paths.root).unwrap().to_path_buf()
				));
			}
		});
	}

	if !errs.is_empty() {
		if errs.is_fatal() || config.errors.on_error == ErrorAction::KILL {
			return Err(errs);
		}

		log::line();
		print_error(errs);
		log::line();
	}
	
	if site_data.stats.active_pages == 0 {
		log::bad!("no files found to squarkup, exiting!");
		return Ok(false);
	} else {
		log::ok!("found {} active files to squarkup", site_data.stats.active_pages);
	}

	// == RENDER == //
	log::is!("rendering...");

	for page in site_data.pages() {
		let r = renderer::render(page, &site_data, &config);

		if let Err(err) = r {
			if err.is_fatal()  || config.errors.on_error == ErrorAction::KILL {
				return Err(err);
			}
			else {
				log::line();
				print_error(err);
				log::line();
			}
		}
	}

	// == SITE DATA == //
	if let Some(ref dest) = config.out.data {
		log::is!("saving site data...");

		let data_raw = site_data.serialise(&config);
		let file = File::create(dest)?;
		serde_json::to_writer_pretty(file, &data_raw).map_err(err!())?;

		log::ok!(slash!("saved site data to {B}{}", dest.to_path_buf()));
	}
	
	Ok(true)
}

fn print_error(err: SquarkError)
{
	match err {
		SquarkError::Recoverable{ msg, hint, debug }
		| SquarkError::Unrecoverable{ msg, hint, debug }
		=> {
			log::bad!(msg);
			for each in debug {
				log::info!(each);
			}
			if !hint.is_empty() {
				log::hint!(hint);
			}
		},
		SquarkError::Multiple{ errs } => {
			for (i, err) in errs.into_iter().enumerate() {
				if i != 0 { log::line(); }
				print_error(err);
			}
		},
		SquarkError::External{ err, msg } => {
			log::bad!(msg);
			log::line();
			println!("{R}{err}");
		},
	}
}
