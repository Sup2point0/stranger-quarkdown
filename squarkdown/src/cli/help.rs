use crate::utils::log;
use crate::utils::colours::*;

use std::process::ExitCode;


pub fn help() -> ExitCode
{
	log::version();
	
	println!(
"{W} Setup your project’s {Y}squarkup.toml{W}:

{F}   › {C}squarkdown {G}init

{W} Run Squarkdown on your project:

{F}   › {C}squarkdown
{F}   › {C}squarkdown {Y}--assets --fonts
{F}   › {C}squarkdown {Y}--quiet
{F}   › {C}squarkdown {Y}--dry-run

{W} Delete Squarkdown’s output for a fresh run:

{F}   › {C}squarkdown {G}clean
{F}   › {C}squarkdown {G}clean {Y}--force
{F}   › {C}squarkdown {G}clean {Y}--dry-run
"
	);

	log::line();

	println!(
"{W}For detailed guidance, please visit the docs!

{W}  GitHub: {B}https://github.com/Sup2point0/stranger-quarkdown/blob/main/docs/reference/cli
{W}  site: {B}https://sup2point0.github.io/stranger-quarkdown/docs/reference/cli
{W}"
	);

	ExitCode::SUCCESS
}
