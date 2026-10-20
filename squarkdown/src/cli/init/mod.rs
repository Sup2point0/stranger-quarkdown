#![allow(unused)]  // TEMP

use crate::colours::*;

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

	wait!()?;
	step! {
		before => "This will initialise your project’s {B}squarkup.toml{W} for you!";
		after => "This will initialise your project’s squarkup.toml for you!";
	}?;

	out!();
	out!("└" => "{C}Thanks for using Stranger Quarkdown, enjoy! 🥕");
	println!();

	Ok(())
}
