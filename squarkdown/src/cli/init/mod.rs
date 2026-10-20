use crate::resolver;
use crate::colours::*;
use crate::macros::*;

use std::io;
use std::process::ExitCode;

mod raw;
mod out; use out::*;
mod wait; use wait::*;
mod step; use step::*;


pub fn init() -> ExitCode
{
	match initialise()
	{
		Ok(..)  => ExitCode::SUCCESS,
		Err(e) => {
			if e.kind() == io::ErrorKind::Interrupted {
				out!();
				out!("└" => "{C}Thanks for using Stranger Quarkdown! 🥕");
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
	println!();

	println!(
		"{GREY} ┌  {C}Welcome to Stranger Quarkdown! {GREY}v{}  ─────────────────────────",
		env!("CARGO_PKG_VERSION")
	);
	out!();
	wait!()?;

	step! {
		before => "Let’s get your project’s {B}squarkup.toml{W} set up!";
		after  => "Let’s get your project’s squarkup.toml set up!";
	}?;

	if let Ok(cwd) = std::env::current_dir() {
		for parent in cwd.ancestors() {
			if let Ok((filepath, _)) = resolver::find_config(&parent) {
				out!();
				out!("{R}×" => "Woah, looks like you already have an existing squarkup config!");
				out!(&slash!("{GREY}found at: {}", filepath));
				out!();
				wait!()?;
			}
		}
	}

	// step! {
	// 	before => ""
	// }?;

	out!();
	out!("└" => "{C}Thanks for using Stranger Quarkdown, enjoy! 🥕");
	println!();

	Ok(())
}
