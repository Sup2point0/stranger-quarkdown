use super::super::ui::*;

use crate::config::*;
use crate::colours::*;

use std::fmt::{ Write };
use std::io;


pub fn configure_error_handling(config: &mut String) -> io::Result<()>
{
	ask!("How should Squarkdown handle non-fatal errors?");
	let on_error = pick_one! {
		ErrorAction::WARN => "Log the error and continue";
		ErrorAction::KILL => "Stop execution";
	}?;
	
	ask!("Enable stricter safety checks?");
	let strict = pick_one! {
		true => ("yes", "default");
		false => ("no", "");
	}?;

	append!(config, "on-error = '{on_error}'");
	append!(config, "strict = '{strict}'");

	Ok(())
}
