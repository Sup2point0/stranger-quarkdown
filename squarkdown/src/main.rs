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

	if let Some(arg) = args.peek_mut() {
		arg.make_ascii_lowercase();

		match arg.as_str()
		{
			  "help"
			| "--help"
			| "-h"
							=> { mode = Mode::HELP;    args.next(); }
			"--version" => { mode = Mode::VERSION; args.next(); }
			"init"      => { mode = Mode::INIT;    args.next(); }
			"clean"     => { mode = Mode::CLEAN;   args.next(); }

			_ => if !arg.starts_with("-") {
				log::bad!("unknown command: {W}{BOLD}{arg}{UNBOLD}");
				return ExitCode::FAILURE;
			}
		}
	}

	match mode
	{
		Mode::HELP => cli::help(),

		Mode::VERSION => {
			println!("Squarkdown v{}", env!("CARGO_PKG_VERSION"));
			ExitCode::SUCCESS
		}

		Mode::SQUARKUP => cli::squarkdown(args),

		Mode::INIT =>
		{
			log::line();
			log::bad!("{W}squarkdown init{R} is not yet supported!{W}");
			log::line();
			// cli::init();
			ExitCode::FAILURE
		}

		Mode::CLEAN => cli::clean(args),
	}
}


