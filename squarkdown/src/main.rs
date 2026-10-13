use squarkdown::cli;
use squarkdown::log;
use squarkdown::colours::*;

use std::process::ExitCode;


/// The Squarkdown command to run.
enum Mode
{
	/// `squarkdown --help`
	HELP,

	/// `squarkdown --version`
	VERSION,

	/// `squarkdown`
	SQUARKUP,

	/// `squarkdown init`
	INIT,

	/// `squarkdown clean`
	CLEAN,
}


fn main() -> ExitCode
{
	let mut args = std::env::args().skip(1).peekable();

	let mut mode = Mode::SQUARKUP;

	if let Some(arg) = args.peek() {
		if arg.eq_ignore_ascii_case("--help")    { mode = Mode::HELP;    } 
		if arg.eq_ignore_ascii_case("-h")        { mode = Mode::HELP;    } 
		if arg.eq_ignore_ascii_case("--version") { mode = Mode::VERSION; } 
		if arg.eq_ignore_ascii_case("init")      { mode = Mode::INIT;    } 
		if arg.eq_ignore_ascii_case("clean")     { mode = Mode::CLEAN;   } 

		if !arg.starts_with("-") {
			log::bad!("unknown command: {W}{BOLD}{arg}{UNBOLD}");
			return ExitCode::FAILURE;
		}
	}

	match mode
	{
		Mode::HELP => {
			println!();
			println!("{P}Squarkdown v{}", env!("CARGO_PKG_VERSION"));
			log::line();
			cli::help()
		}

		Mode::VERSION =>
		{
			println!("Squarkdown v{}", env!("CARGO_PKG_VERSION"));
			ExitCode::SUCCESS
		}

		Mode::SQUARKUP =>
		{
			println!();
			println!("{P}Squarkdown v{}", env!("CARGO_PKG_VERSION"));
			log::line();
			cli::squarkdown(args)
		}

		Mode::INIT =>
		{
			log::line();
			log::bad!("{W}squarkdown init{R} is not yet supported!{W}");
			log::line();
			ExitCode::FAILURE
		}

		Mode::CLEAN =>
		{
			log::line();
			log::bad!("{W}squarkdown clean{R} is not yet supported!{W}");
			log::line();
			ExitCode::FAILURE
		}
	}
}


