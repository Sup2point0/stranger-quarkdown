use squarkdown::*;
use squarkdown::utils::log;
use squarkdown::utils::colours::*;

use std::fs::File;


fn main() -> Result<(), ()>
{
	println!();
	println!("{PINK}Squarkdown v{}", "4.0");
	log::line();
	log::is!("squarking up...");

	match squarkup()
	{
		Ok(_) => {
			log::line();
			println!("{PINK}squarkup finished!");
			Ok(())
		},
		Err(e) => {
			log::line();
			print_error(e);
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
	log::ok!("found your project: {BLUE}{}", project_root.display());

	let config = resolver::resolve_config(project_root)?;
	log::ok!("found your site: {BLUE}{}", config.paths.site.display());
	
	let files = resolver::find_files(&config)?;
	
	if files.is_empty() {
		log::bad!("No files found to squarkup, exiting!");
		return Ok(false);
	} else {
		log::ok!("Found ")
	}
	
	let mut site_data = SiteData::new();
	
	for filepath in files {
		let file = File::open(&filepath).map_err(err!())?;
		let mut parser = CharmParser::init(file, Some(filepath.clone())).map_err(err!())?;
		
		if let Some(page_data) = parser.parse(&config).unwrap() {
			log::info!("found active file: {BLUE}{filepath:?}");
			site_data.add_page(filepath, page_data);
		}
	}
	
	for page in site_data.pages.values() {
		let dest = config.paths.root.join(&page.destination);
		let source = File::open(&page.filepath).map_err(err!())?;
		let target = File::create(dest).map_err(err!())?;

		let mut renderer = Renderer::init(source, target);
		renderer.render(&config);  // FIXME
	}
	
	Ok(true)
}

fn print_error(err: SquarkError)
{
	match err {
		SquarkError::Recoverable { msg } => log::bad!(msg),
		SquarkError::ManyRecoverable { errs } => {
			for err in errs {
				print_error(err);
				log::line();
			}
		},
		SquarkError::Unrecoverable { msg, hint, debug } => {
			log::bad!(msg);
			log::hint!(hint);
			for each in debug {
				log::info!(each);
			}
		},
		SquarkError::External(e) => log::bad!(e),
	}
}
