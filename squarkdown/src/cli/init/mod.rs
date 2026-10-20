use crate::resolver;
use crate::colours::*;
use crate::macros::*;

use path_macro::path;

use std::fs;
use std::io;
use std::path::{ Path };
use std::process::ExitCode;

mod raw;
mod out; use out::*;
mod wait; use wait::*;
mod step; use step::*;
mod pick; use pick::*;


pub fn init() -> ExitCode
{
	match initialise()
	{
		Ok(..)  => ExitCode::SUCCESS,
		Err(e) => {
			if e.kind() == io::ErrorKind::Interrupted {
				out!();
				out!("└" => "{G}Thanks for using Stranger Quarkdown! 🥕");
				println!();
				return ExitCode::SUCCESS;
			}
			else {
				println!("{e:?}");
				return ExitCode::FAILURE;
			}
		}
	}
}

fn initialise() -> io::Result<()>
{
	let cwd = std::env::current_dir()?;

	println!();

	println!(
		"{GREY} ┌  {P}Welcome to Stranger Quarkdown! {GREY}v{}  ─────────────────────────",
		env!("CARGO_PKG_VERSION")
	);
	out!();
	wait!()?;

	step! {
		before => "Let’s get your project’s {B}squarkup.toml{W} set up!";
		after  => "Let’s get your project’s squarkup.toml set up!";
	}?;
	step! {
		before => "Heads up, Squarkdown is for {Y}SvelteKit{W} + {Y}MDsveX{W} projects.";
		after  => "Heads up, Squarkdown is for SvelteKit + MDsveX projects.";
	}?;

	check_existing_config(&cwd)?;

	out!();
	out!("├" => "{Y}Configuring Paths");
	out!();
	configure_site(&cwd)?;

	out!();
	out!("Writing your {B}squarkup.toml...");
	wait!()?;
	print!("{PREV}{CLEAR}");
	out!("Wrote your squarkup.toml!");

	step! {
		before => "For more help and guidance, check the docs at {C}https://sup2point0.github.io/stranger-quarkdown/docs{W}!";
		after => "For more help and guidance, check the docs at https://sup2point0.github.io/stranger-quarkdown/docs!";
	}?;

	out!();
	out!("{GREY}└" => "{P}Thanks for using Stranger Quarkdown, enjoy! 🥕");
	println!();

	Ok(())
}

fn check_existing_config(cwd: &Path) -> io::Result<()>
{
	for parent in cwd.ancestors() {
		if let Ok((filepath, _)) = resolver::find_config(&parent) {
			out!();
			out!("{R}×" => "Woah, looks like you already have an existing squarkup config!");
			out!(&slash!("{GREY}found at: {GREY1}{}", filepath));
			out!();
			
			ask!("Exit?");
			let should_quit = pick_one! {
				true  => ("yes", "");
				false => ("no",  "changes may overwrite existing files");
			}?;

			if should_quit {
				return Err(io::Error::from(io::ErrorKind::Interrupted));
			}
			else {
				break;
			}
		}
	}

	Ok(())
}

fn configure_site(cwd: &Path) -> io::Result<()>
{
	let mut candidates = vec![];

	for folder in fs::read_dir(cwd)? {
		let folder = folder?;

		if folder.file_type().is_ok_and(|t| t.is_dir()) {
			let path = folder.path();

			if path!(path / "src" / "app.html").exists() {
				candidates.push(path);
			}
		}
	}

	if candidates.is_empty() {
		ask!("Could not find your SvelteKit folder. How to proceed?");
		let choice = pick_one! {
			"create" => "Squarkdown will create a new folder for your SvelteKit project";
			"skip" => "Skip this question";
		}?;
	}
	else {
		ask!("Which of these contains your SvelteKit project?");
	}

	Ok(())
}
