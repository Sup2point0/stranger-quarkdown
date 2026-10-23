use super::super::shared::*;
use super::super::ui::*;

use crate::config::*;

use std::fmt::{ Write };
use std::io;


pub fn configure_error_handling(config: &mut String) -> io::Result<()>
{
	let on_error = pick_one!(
		ask: "How should Squarkdown handle non-fatal errors?";
		choices: {
			ErrorAction::WARN => "Log the error and continue";
			ErrorAction::KILL => "Stop execution";
		}
	)?;
	
	let strict = pick_one!(
		ask: "Enable stricter safety checks?";
		choices: {
			true => ("yes", "default");
			false => ("no", "");
		}
	)?;

	append!(config, "on-error = '{on_error}'");
	append!(config, "strict = {strict}");

	Ok(())
}
