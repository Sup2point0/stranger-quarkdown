use crate::log;
use crate::colours::*;

use std::process::ExitCode;


pub fn help() -> ExitCode
{
	log::version();
	
	println!(
"{W} Setup your project's {Y}squarkup.toml{W}:

{GREY}   › {C}squarkdown {G}init

{W} Run Squarkdown on your project:

{GREY}   › {C}squarkdown

{W} Run Squarkdown on your project, with extras enabled too:

{GREY}   › {C}squarkdown {Y}--assets --fonts

{W} Simulate a run of Squarkdown without actual output:

{GREY}   › {C}squarkdown {Y}--dry-run

{W} Delete Squarkdown’s output for a fresh run:

{GREY}   › {C}squarkdown {G}clean
"
	);

	log::line();

	println!(
"{W}For detailed guidance, please visit the docs!

{W}  GitHub: {B}https://github.com/Sup2point0/stranger-quarkdown/tree/main/docs
{W}  site: {B}https://sup2point0.github.io/stranger-quarkdown/docs
{W}"
	);

	ExitCode::SUCCESS
}
