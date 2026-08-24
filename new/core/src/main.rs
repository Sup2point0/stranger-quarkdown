use squarkdown::*;
use squarkdown::utils::log;
use squarkdown::utils::colours::*;

use std::fs::File;


fn main() -> Result<(), ()>
{
	println!("{PINK}Squarkdown v{}", "4.0");
	println!("{GREY}─────────────────────");
	log::is!("squarking up...");

	match squarkup()
	{
		Ok(_) => {
			println!("{GREY}─────────────────────");
			println!("{PINK}squarkup finished!");
			Ok(())
		},
		Err(e) => {
			log::bad!(e);
			println!("{GREY}─────────────────────");
			println!("{RED}squarkup failed!");
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
fn squarkup() -> Result<bool, Box<dyn std::error::Error>>
{
	let project_root = resolver::resolve_project_root()?;
	log::ok!("found your project: {BLUE}{}", project_root.display());

	let config = resolver::resolve_config(project_root)?;
	
	let files = resolver::find_files(&config)?;
	
	if files.is_empty() {
		log::bad!("No files found to squarkup, exiting!");
		return Ok(false);
	} else {
		log::ok!("Found ")
	}
	
	let mut site_data = SiteData::new();
	
	for filepath in files {
		let file = File::open(&filepath)?;
		let mut parser = CharmParser::init(file, Some(filepath.clone()))?;
		
		if let Some(page_data) = parser.parse(&config)? {
			log::info!("found active file: {BLUE}{filepath:?}");
			site_data.add_page(filepath, page_data);
		}
	}
	
	for page in site_data.pages.values() {
		let dest = config.paths.root.join(&page.destination);
		let source = File::open(&page.filepath)?;
		let target = File::create(dest)?;

		let mut renderer = Renderer::init(source, target);
		renderer.render(&config)?;
	}
	
	Ok(true)
}
