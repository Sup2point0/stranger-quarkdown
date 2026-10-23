use super::super::shared::*;
use super::super::ui::*;

use crate::colours::*;

use std::fmt::{ Write };
use std::io;


pub fn configure_project(config: &mut String) -> io::Result<()>
{
	configure_name(config)?;
	configure_github(config)?;

	Ok(())
}

fn configure_name(config: &mut String) -> io::Result<()>
{
	if let Some(project_name) = input!("What displayed name should Squarkdown use for your project?")?
		&& !project_name.is_empty()
	{
		append!(config, "name = {project_name:?}");
	} else {
		append!(config, "name = \"\"  # unconfigured");
	}
	Ok(())
}

fn configure_github(config: &mut String) -> io::Result<()>
{
	let mut answer = None;
	
	if let Some(github) = input!(
		"What’s the GitHub ID of your project? {GREY}(e.g. Sup2point0/stranger-quarkdown)"
	)?
	{
		if is_valid_github(&github) {
			answer = Some(github);
		}
		else {
			while let Some(github) = input!(
				"Invalid GitHub ID, try again? {GREY}(e.g. Sup2point0/stranger-quarkdown)"
			)?
			{
				if is_valid_github(&github) {
					answer = Some(github);
					break;
				}
			}
		}
	}

	if let Some(github) = answer {
		append!(config, "github = {github:?}");
	} else {
		append!(config, "github = \"\"  # unconfigured");
	}

	Ok(())
}

fn is_valid_github(github_id: &str) -> bool
{
	github_id.chars().filter(|&c| c == '/').count() == 1
}
