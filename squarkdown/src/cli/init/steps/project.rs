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
	
	let github = input!("What’s the GitHub ID of your project? (e.g. Sup2point0/stranger-quarkdown)")?;
	if !github.is_empty() {
		append!(config, "github = '{github}'");
	}

	Ok(())
}
