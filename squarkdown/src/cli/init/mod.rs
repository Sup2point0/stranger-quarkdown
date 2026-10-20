use crate::colours::*;

use std::io;
use std::process::ExitCode;

mod wait; use wait::*;
// mod step; use step::step;


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
		"{GREY} ┌ {C}Welcome to Stranger Quarkdown! {GREY}v{} ─────────────────────────",
		env!("CARGO_PKG_VERSION")
	);

	wait!()?;
	println!("FIRST");
	wait!()?;
	println!("SECOND");
	wait!()?;
	println!("THIRD");

	Ok(())
}
