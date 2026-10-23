use super::super::shared::*;
use super::super::ui::*;

use crate::utils::colours::*;

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
	if let Some(project_name) = input!(
		before: "What displayed name should Squarkdown use for your project? {GREY}(capitalisation will be preserved)";
		after: "What displayed name should Squarkdown use for your project?";
	)?
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
		before: "What’s the GitHub ID of your project? {GREY}(in {GREY1}user/project{GREY} format, e.g. {GREY1}Sup2point0/stranger-quarkdown{GREY})";
		after: "What’s the GitHub ID of your project?";
	)?
	{
		if is_valid_github(&github) {
			answer = Some(github);
		}
		else {
			while let Some(github) = input!(
				before: "Invalid GitHub ID! Try again? {GREY}(provide {GREY1}user/project{GREY}, e.g. {GREY1}Sup2point0/stranger-quarkdown{GREY})";
				after: "Invalid GitHub ID! Try again?";
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
