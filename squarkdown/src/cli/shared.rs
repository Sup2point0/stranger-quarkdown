use crate::utils::log;
use crate::utils::colours::*;
use crate::utils::macros::*;


pub mod cli_flags
{
	use std::sync::OnceLock;

	/// Hide debug output from [`info!`](crate::utils::log::info)? (enabled by `--quiet`)
	pub static NO_INFO: OnceLock<()> = OnceLock::new();

	/// Hide warnings from [`warns!`](crate::utils::log::warns)? (enabled by `--no-warn`)
	pub static NO_WARN: OnceLock<()> = OnceLock::new();

	/// Collapse errors when logging? (enabled by `--compact`)
	pub static COLLAPSE_ERRORS: OnceLock<()> = OnceLock::new();
}


/// Print an error showing unknown `args`.
pub(super) fn unknown_args(args: &[String])
{
	if args.len() == 1 {
		log::bad!("unknown option: {W}{BOLD}{}{UNBOLD}", args[0]);
	} else {
		log::bad!("unknown options: {W}{BOLD}{}{UNBOLD}",
			args.join(&fmt!(" {R}{UNBOLD}·{BOLD}{W} "))
		);
	}
}
