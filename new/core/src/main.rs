use squarkdown::{
	resolver,
	SiteData, CharmParser, Renderer,
	utils::log,
	utils::colours::*,
	utils::macros::*,
};

use std::fs::File;


fn main() -> Result<(), ()>
{
	println!("{PINK}Squarkdown v{}", "4.0");
	println!("{GREY}—————————————————————");
	log::is!("squarking up...");

	match squarkup()
	{
		Ok(_) => {
			log::ok!("squarkup finished!");
			println!("{GREY}—————————————————————");
			Ok(())
		},
		Err(e) => {
			log::bad!(e);
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
fn squarkup() -> Result<bool, String>
{
	let project_root = resolver::resolve_project_root()?;
	let config = resolver::load_config(project_root);
	
	let files = resolver::find_files(&config);
	
	if files.is_empty() {
		log::bad!("No files found to squarkup, exiting!");
		return Ok(false);
	} else {
		log::ok!("Found ")
	}
	
	let mut site_data = SiteData::new();
	
	for filepath in files {
		let file = File::open(filepath.clone())?;
		let mut parser = CharmParser::init(file, Some(filepath.clone()))?;
		
		if let Some(page_data) = parser.parse(&config)? {
			log::info!("found active file: {BLUE}{filepath:?}");
			site_data.add_page(filepath, page_data);
		}
	}
	
	for page in site_data.pages.values() {
		let dest = dir!(config.paths.root / &page.destination);
		let source = File::open(page.filepath.clone())?;
		let target = File::create(dest)?;

		let mut renderer = Renderer::init(source, target);
		renderer.render(&config)?;
	}
	
	Ok(true)
}
