use super::super::ui::*;

use crate::utils;
use crate::colours::*;
use crate::macros::*;

use path_macro::path;

use std::fmt::{ Write };
use std::io;
use std::path::{ Path };


pub fn configure_site(config: &mut String, cwd: &Path) -> io::Result<()>
{
	let mut folders = vec![];
	let mut candidates = vec![];

	utils::for_folders(cwd, |folder| {
		let path = folder.path();
		let mut path_rel = utils::display_rel(&path, cwd);
		
		if !path_rel.ends_with('/') {
			path_rel += "/";
		}

		if path!(path / "src" / "app.html").exists() {
			candidates.push(path_rel.clone());
		}
		folders.push(path_rel);
	})?;

	if !candidates.is_empty() {
		candidates.push(str!("None of the above"));

		ask!("Which of these contains your SvelteKit project?");
		let choice = pick_one!(candidates)?;

		if choice != "None of the above" {
			append!(config, "site = '{choice}'");
			return Ok(());
		}

		ask!("How to proceed?");
	}
	else {
		ask!("Could not find an existing SvelteKit folder. How to proceed?");
	}

	choices! { PICK, CREATE, SKIP = "Skip this question" }

	let choice = pick_one! {
		PICK   => "Pick an existing directory";
		CREATE => "Create a new directory";
		SKIP   => "";
	}?;

	if choice == SKIP { return Ok(()); }

	if choice == PICK {
		folders.push(str!("None of the above"));

		ask!("Which folder contains your SvelteKit project?");
		let choice = pick_one!(folders)?;

		if choice != "None of the above" {
			append!(config, "site = '{choice}/'");
			return Ok(());
		}

		// If the user picks "None of the above", fall back to manually entering
	}

	let mut folder = input!("Enter the folder name for your SvelteKit project:")?;

	if folder.starts_with('/') {
		folder.remove(0);
	} else if folder.starts_with("./") {
		folder.replace_range(0..2, "");
	}

	if !folder.ends_with('/') {
		folder += "/";
	}

	append!(config, "site = '{folder}'");

	Ok(())
}

pub fn configure_sources(config: &mut String, cwd: &Path) -> io::Result<()>
{
	let mut candidates = vec![
		(str!("/"), "project root (non-recursive)")
	];

	utils::for_folders(cwd, |folder| {
		let mut path_rel = utils::display_rel(folder.path(), cwd);
		path_rel += "/";
		candidates.push((path_rel, ""));
	})?;

	ask!("Where should Squarkdown look for .md files?");
	let Some(choices) = pick_many_or_skip!(candidates)? else {
		return Ok(());
	};

	append!(config, "sources = [\n\t'{}'\n]", choices.join("',\n\t'"));

	Ok(())
}
