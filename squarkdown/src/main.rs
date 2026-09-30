use squarkdown::*;
use squarkdown::prelude::*;
use squarkdown::colours::*;

use std::fs::File;
use std::io::BufWriter;
use std::process::ExitCode;
use std::time::Instant;


fn main() -> ExitCode
{
	println!();
	println!("{P}Squarkdown v{}", env!("CARGO_PKG_VERSION"));
	log::line();

	let args: Vec<String> = std::env::args().collect();

	if args.iter().any(|arg| arg == "--help") {
		help();
		return ExitCode::SUCCESS;
	}

	let t_init = Instant::now();
	let r = squarkup(&args);
	let t = t_init.elapsed();
	let perf = t.as_secs_f64() * 1000.0;

	match r
	{
		Ok(()) => {
			log::line();
			println!("{P}squarkup finished! {GREY}{perf:.2?} ms{W}");
			ExitCode::SUCCESS
		},
		Err(e) => {
			log::error(e);
			println!("{R}squarkup failed! {GREY}{perf:.2?} ms\n{W}");
			ExitCode::FAILURE
		},
	}
}

/// Run Squarkdown on the user's project.
/// 
/// This includes squarkup as well as extras like fonts and assets preprocessing.
fn squarkup(args: &[String]) -> SquarkResult
{
	/* NOTE: We're intentionally keeping the main pipeline under one scope so all the shared variables are easily accessible instead of requiring a whole load of messy parameter-passing. Some loss in readability, but gains in concision ;) */

	// == SETUP == //
	let project_root = resolver::resolve_project_root()?;
	log::ok!(slash!("found your project: {B}{}", project_root));

	let config = resolver::resolve_config(&project_root)?;
	log::ok!(slash!("found your site: {B}{}", config.paths.site));

	let mut site_data = SiteData::new();

	// == FONTS == //
	if args.iter().any(|arg| arg == "--fonts") {
		log::is!("injecting fonts queries...");

		extras::prep_fonts(&config)
			.or_else(|e| e.depends(&config))?;

		log::ok!("injected fonts queries");
	}

	// == ASSETS == //
	if args.iter().any(|arg| arg == "--assets") {
		log::is!("preprocessing assets...");

		extras::prep_assets(&mut site_data, &config)
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
				
				log::info!(slash!(
					"found active file: {}/{GREY1}{}",
					filepath.parent().unwrap().strip_prefix(&config.paths.root).unwrap(),
					filepath.file_name().unwrap().to_string_lossy(),
				));
			}
		}.or_else(|e| e.depends(&config))?;
	}
	
	if site_data.stats.checked_files == 0 {
		SquarkError::Recoverable {
			msg: str!("no files found to squarkup"),
			hint: fmt!("check your {Y}paths.sources{G}, {Y}paths.include{G}, {Y}paths.exclude{G} are configured correctly?"),
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
		log::is!("rendering...");

		let mut rendered = 0;

		for page in site_data.pages() {
			renderer::render(page, &site_data, &config)
				.or_else(|e| e.depends(&config))?;

			rendered += 1;
		}

		log::ok!("rendered {rendered}/{} pages", site_data.stats.active_pages);
	}

	// == SITE DATA == //
	if let Some(dest) = &config.out.site_data_path {
		log::is!("saving site data...");

		let data_raw = site_data.serialise(&config);
		let file = BufWriter::new(File::create(dest)?);
		serde_json::to_writer_pretty(file, &data_raw)?;

		log::ok!(slash!("saved site data to: {B}{}", dest));
	}
	
	Ok(())
}

fn help()
{
	println!(
"{W} Setup your project's {Y}squarkup.toml{W}:

{GREY}  › {C}squarkdown {G}init

{W} Run Squarkdown on your project:

{GREY}  › {C}squarkdown

{W} Run Squarkdown on your project, with extras enabled too:

{GREY}  › {C}squarkdown {Y}--assets --fonts
"
	);

	log::line();

	println!(
"{W}For detailed guidance, please visit the docs!

{W}  GitHub: {B}https://github.com/Sup2point0/stranger-quarkdown/tree/main/docs
{W}  site: {B}https://sup2point0.github.io/stranger-quarkdown/docs
"
	);

}
