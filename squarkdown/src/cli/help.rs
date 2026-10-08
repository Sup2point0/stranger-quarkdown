use crate::log;
use crate::colours::*;

use std::process::ExitCode;


pub fn help() -> ExitCode
{
	println!(
"{W} Setup your project's {Y}squarkup.toml{W}:

{GREY}  › {C}squarkdown {G}init

{W} Run Squarkdown on your project:

{GREY}  › {C}squarkdown

{W} Run Squarkdown on your project, with extras enabled too:

{GREY}  › {C}squarkdown {Y}--assets --fonts
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
