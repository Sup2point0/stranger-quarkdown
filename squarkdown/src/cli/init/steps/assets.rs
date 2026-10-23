use super::super::shared::*;
use super::super::ui::*;

use crate::utils;
use crate::macros::*;

use std::fmt::{ Write };
use std::io;
use std::path::{ Path };


pub fn configure_assets(config: &mut String, cwd: &Path) -> io::Result<()>
{
	match _configure_assets_(config, cwd)
	{
		Ok(()) => Ok(()),
		Err(AbandonError::Other(e)) => Err(e),
		Err(AbandonError::ABANDON) => {
			config.clear();
			Ok(())
		}
	}
}

fn _configure_assets_(config: &mut String, cwd: &Path) -> AbandonOrResult<io::Error>
{
	configure_assets_folder(config, cwd)?;
	Ok(())
}

fn configure_assets_folder(config: &mut String, cwd: &Path) -> AbandonOrResult<io::Error>
{
	let mut candidates = vec![];

	utils::for_folders(cwd, |folder| {
		let mut path_rel = utils::display_rel(folder.path(), cwd);
		path_rel += "/";
		candidates.push((path_rel, ""));
	})?;

	candidates.push((str!("Cancel"), "skip this feature"));

	if let Some(choice) = pick_one_or_skip!(
		ask: "Where should Squarkdown look for assets?";
		choices: candidates
	)? {
		if *choice == "Cancel" {
			return Err(AbandonError::ABANDON);
		}

		append!(config, "folder = '{choice}'");
	} 

	Ok(())
}
