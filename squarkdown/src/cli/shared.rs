use crate::utils::log;
use crate::utils::colours::*;
use crate::utils::macros::*;


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
