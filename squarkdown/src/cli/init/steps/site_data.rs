use super::super::shared::*;
use super::super::ui::*;

use crate::utils::macros::*;
use crate::utils::colours::*;

use std::fmt::{ Write };
use std::io;


pub fn configure_site_data(config: &mut String, site: &str) -> io::Result<()>
{
	let Some(mut filepath) = pick_one_or_skip!(
		ask: "Where should Squarkdown export your site data?";
		choices: {
			/* NOTE: `site` has trailing `/` */
			fmt!("{site}src/site.json")      => "";
			fmt!("{site}src/site-data.json") => "";
			fmt!("{site}src/data/site.json") => "";
			fmt!("Enter manually")           => "";
		}
	)? else {
		return Ok(());
	};

	if filepath == "Enter manually" {
		let Some(answer) = input!("Where should Squarkdown export your site data?")?
			else { return Ok(()) };

		filepath = answer;
	}

	append!(config, "site-data-path = {filepath:?}");

	Ok(())
}
