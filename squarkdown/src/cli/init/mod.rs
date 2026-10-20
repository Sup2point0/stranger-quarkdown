#![allow(unused)]  // TEMP

use crate::colours::*;

use std::io;
use std::process::ExitCode;

mod out;
mod wait; use wait::*;
mod step; use step::*;


pub fn init() -> ExitCode
{
	match initialise()
	{
		Ok(..)  => ExitCode::SUCCESS,
		Err(..) => ExitCode::FAILURE,
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
	step!("This will initialise your project’s {B}squarkup.toml{W} for you!")?;

	Ok(())
}
