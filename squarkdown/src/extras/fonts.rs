use crate::prelude::*;
use crate::colours::*;
use crate::macros::*;

use path_macro::path;
use regex::regex;

use std::fs;


pub fn prep_fonts(config: &SquarkupConfig) -> SquarkResult
{
	if config.fonts.queries.is_empty() {
		return Err(SquarkError::Recoverable {
			msg: fmt!("no queries provided in {W}fonts.queries"),
			hint: str!(),
			debug: vec![],
		});
	}

	let path = path!(&config.paths.site / "src/app.html");

	if !path.is_file() {
		return Err(SquarkError::Recoverable {
			msg: fmt!("could not find your {W}app.html"),
			hint: fmt!("Squarkdown injects Google Fonts queries into the {W}<head>{G} of your {W}app.html"),
			debug: vec![
				slash!("{GREY1}{}{GREY} is not a file", path),
			],
		});
	}

	let query = fmt!(
		"css2?family={queries}&display=swap",
		queries = config.fonts.queries.join("&family="),
	);

	let before = fs::read_to_string(&path)?;

	let after = {
		if before.contains("fonts.googleapis.com/css2?") && before.contains("display=swap") {
			regex!("css2.*display=swap").replace(&before, query)
		} else {
			regex!("( *)</head>").replace(&before,
				fmt!("\n$1  <link rel=\"stylesheet\" href=\"https://fonts.googleapis.com/{query}\" />\n$1</head>")
			)
		}
	};

	fs::write(path, after.as_bytes())?;
	Ok(())
}
