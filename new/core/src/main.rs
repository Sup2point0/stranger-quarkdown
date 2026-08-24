use squarkdown::{
	resolver,
	SiteData, CharmParser, Renderer,
	utils::log,
};

use std::fs::File;


fn main() -> Result<(), Box<dyn std::error::Error>>
{
	let project_root = resolver::resolve_project_root();
	let config = resolver::load_config(project_root);
	
	let files = resolver::find_files(&config);
	
	if files.is_empty() {
		log::bad!("No files found to squarkup, exiting!");
		return Ok(());
	}
	
	let site_data = SiteData::new();
	
	for filepath in files {
		let file = File::open(filepath)?;
		let mut parser = CharmParser::init(file, Some(filepath))?;
		
		if let Some(page_data) = parser.parse(&config)? {
			log::ok!("found active file: {BLUE}{filepath}");
			site_data.add_page(filepath, page_data);
		}
	}
	
	for page in site_data.pages.values() {
		let renderer = Renderer::init(File::open(page.filepath));
		
		renderer.render()?;
	}
	
	Ok(())
}
