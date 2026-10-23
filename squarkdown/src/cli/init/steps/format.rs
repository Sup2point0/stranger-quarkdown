use super::super::shared::*;
use super::super::ui::*;

use crate::config::FormatConfig;
use crate::utils::colours::*;

use std::fmt::{ Write };
use std::io;


pub fn configure_formatting(config: &mut String, defaults: &FormatConfig) -> io::Result<()>
{
	let inject_head = pick_one!(
		before: "Should Squarkdown inject metadata inside {Y}<svelte:head>{W}?";
		after: "Should Squarkdown inject metadata inside <svelte:head>?";
		choices: {
			true => "yes" => if defaults.inject_head {"default"} else {""};
			false => "no" => if !defaults.inject_head {"default"} else {""};
		}
	)?;
	
	append!(config, "inject-head = {inject_head}");

	Ok(())
}
