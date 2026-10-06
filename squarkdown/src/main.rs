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
}


fn main() -> ExitCode
{
	let mut mode   = Mode::SQUARKUP;
	let mut assets = false;
	let mut fonts  = false;

	let mut found_unknown = false;

	for arg in std::env::args().skip(1) {
		match arg.as_str()
		{
			"--help" | "-h" => {
				mode = Mode::HELP;
				break;
			}
			"--version" => {
				mode = Mode::VERSION;
				break;
			}
			"--assets" => assets = true,
			"--fonts"  => fonts = true,
			f if f.starts_with("-") => {
				log::bad!("unknown CLI option: {f}");
				found_unknown = true;
			}
			_ => mode = Mode::HELP,
		}
	}

	if found_unknown {
		return ExitCode::FAILURE;
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
			println!("{P}Squarkdown v{}", env!("CARGO_PKG_VERSION"));
			ExitCode::SUCCESS
		}

		Mode::SQUARKUP =>
		{
			println!();
			println!("{P}Squarkdown v{}", env!("CARGO_PKG_VERSION"));
			log::line();
			cli::squarkdown(assets, fonts)
		}
	}
}


