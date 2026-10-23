use super::super::shared::*;
use super::super::ui::*;

use crate::colours::*;

use std::fmt::{ Write };
use std::io;


pub fn configure_project(config: &mut String) -> io::Result<()>
{
	let project_name = input!("What displayed name should Squarkdown use for your project?")?;
	if !project_name.is_empty() {
		append!(config, "name = '{project_name}'");
	}
	
	let mut github = input!("What’s the GitHub ID of your project? (e.g. Sup2point0/stranger-quarkdown)")?;

	while !github.is_empty() {
		if github.chars().filter(|&c| c == '/').count() == 1 {
			append!(config, "github = '{github}'");
			break;
		}

		github = input!("Invalid GitHub ID, try again? {GREY}(e.g. Sup2point0/stranger-quarkdown)")?;
	}

	Ok(())
}
