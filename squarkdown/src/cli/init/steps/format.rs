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
	
	let preserve_heading = pick_one!(
		before: "Should Squarkdown preserve the {Y}# Heading{W} of each {B}.md{W} file?";
		after: "Should Squarkdown preserve the # Heading of each .md file?";
		choices: {
			true => "yes" => if defaults.preserve_heading {"default"} else {""};
			false => "no" => if !defaults.preserve_heading {"default"} else {""};
		}
	)?;
	
	let preserve_comments = pick_one!(
		before: "Should Squarkdown preserve comments in {B}.md{W} files?";
		after: "Should Squarkdown preserve comments in .md files?";
		choices: {
			true => "yes" => if defaults.preserve_comments {"default"} else {""};
			false => "no" => if !defaults.preserve_comments {"default"} else {""};
		}
	)?;

	append!(config, "inject-head = {inject_head}");
	append!(config, "preserve-heading = {preserve_heading}");
	append!(config, "preserve-comments = {preserve_comments}");

	Ok(())
}
