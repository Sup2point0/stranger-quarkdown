use super::super::shared::*;
use super::super::ui::*;

use crate::utils;
use crate::utils::macros::*;

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
	configure_assets_extensions(config)?;
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
		append!(config, "folder = {choice:?}");
	} 

	Ok(())
}

fn configure_assets_extensions(config: &mut String) -> AbandonOrResult<io::Error>
{
	let Some(mut choices) = pick_many_or_skip!(
		ask: "What extensions do your assets have?";
		choices: {
			[true]  "png" => "";
			[true]  "jpg" => "";
			[true]  "webp" => "";
			[true]  "svg" => "";
			[false] "gif" => "";
			[false] "mp3" => "";
			[false] "mp4" => "";
			[false] "pdf" => "";
			[false] "ttf" => "";
		}
	)? else {
		return Ok(());
	};

	if let Some(i) = choices.iter().position(|&ext| ext == "jpg") {
		choices.insert(i + 1, "jpeg");
	}

	append!(config, "extensions = [");
	append!(config, "\t\"{}\"", choices.join("\",\n\t\""));
	append!(config, "]");

	Ok(())
}
