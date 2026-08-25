use squarkdown::*;
use squarkdown::utils::log;
use squarkdown::utils::colours::*;

use std::fs::File;
use std::time::Instant;


fn main() -> Result<(), ()>
{
	println!();
	println!("{PINK}Squarkdown v{}", "4.0");
	log::line();
	log::is!("squarking up...");

	let t_init = Instant::now();

	match squarkup()
	{
		Ok(_) => {
			let t = t_init.elapsed();
			log::line();
			println!("{PINK}squarkup finished! {GREY}{:.2?} ms", t.as_secs_f64() * 1000.0);
			Ok(())
		},
		Err(e) => {
			log::line();
			print_error(e);
			log::line();
			println!("{RED}squarkup failed!");
			println!();
			Err(())
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
	let project_root = resolver::resolve_project_root()?;
	log::ok!(slash!("found your project: {BLUE}{}", project_root));

	let config = resolver::resolve_config(project_root)?;
	log::ok!(slash!("found your site: {BLUE}{}", config.paths.site));

	let mut site_data = SiteData::new();
	let mut found_active_file = false;
	
	for filepath in resolver::resolve_files(&config) {
		let filepath = filepath.unwrap();

		let file = File::open(&filepath).map_err(err!())?;
		found_active_file = true;

		let mut parser = CharmParser::init(file, Some(filepath.clone())).map_err(err!())?;
		
		if let Some(page_data) = parser.parse(&config).unwrap() {
			log::info!(slash!("found active file: {GREY_LIGHT}{}", filepath.strip_prefix(&config.paths.root).unwrap().to_path_buf()));
			site_data.add_page(filepath, page_data);
		}
	}
	
	if !found_active_file {
		log::bad!("No files found to squarkup, exiting!");
		return Ok(false);
	}

	// for page in site_data.pages.values() {
	// 	let dest = config.paths.root.join(&page.destination);
	// 	let source = File::open(&page.filepath).map_err(err!())?;
	// 	let target = File::create(dest).map_err(err!())?;

	// 	let mut renderer = Renderer::init(source, target);
	// 	renderer.render(&config);  // FIXME
	// }
	
	Ok(true)
}

fn print_error(err: SquarkError)
{
	match err {
		SquarkError::Recoverable { msg, hint, debug }
		| SquarkError::Unrecoverable { msg, hint, debug } => {
			log::bad!(msg);
			for each in debug {
				log::info!(each);
			}
			log::hint!(hint);
		},
		SquarkError::Multiple { errs } => {
			for (i, err) in errs.into_iter().enumerate() {
				if i != 0 { log::line(); }
				print_error(err);
			}
		},
		SquarkError::External(e) => log::bad!(e),
	}
}
