use super::super::ui::*;

use crate::config::*;
use crate::utils;
use crate::colours::*;
use crate::macros::*;

use std::fmt::{ Write };
use std::io;
use std::path::{ Path };


pub fn configure_error_handling(config: &mut String, cwd: &Path) -> io::Result<()>
{
	ask!("How should Squarkdown handle non-fatal errors?");
	let choice = pick_one! {
		ErrorAction::WARN => "Log the error and continue";
		ErrorAction::KILL => "Stop execution";
	}?;
	append!(config, "on-error = '{choice}'");

	Ok(())
}
