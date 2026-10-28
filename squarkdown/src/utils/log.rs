use crate::prelude::*;
use crate::cli::QUIET;
use crate::utils::colours::*;

use std::fmt::Display;


/// Print a line to the console.
pub fn line() { println!("{F}─────────────────────────────"); }

/// Print Squarkdown’s version to the console.
pub fn version()
{
	println!();
	println!("{P}Squarkdown v{}", env!("CARGO_PKG_VERSION"));
	line();
}


/// Log a generic informative message with no special colouring, which can be quickly skimmed past.
#[macro_export] macro_rules! info {
	($indent:expr => $($args:tt)*) => { $crate::utils::log::log_info($indent, $crate::fmt!($($args)*)) };
	($($args:tt)*) => { $crate::info!(0 => $($args)*) };
} pub use info;

/// Log an action that Squarkdown is about to perform, to set up expectations.
#[macro_export] macro_rules! is {
	($indent:expr => $($args:tt)*) => { $crate::utils::log::log_is($indent, $crate::fmt!($($args)*)) };
	($($args:tt)*) => { $crate::is!(0 => $($args)*) };
} pub use is;

/// Log a checkpoint that has successfully been reached.
#[macro_export] macro_rules! ok {
	($indent:expr => $($args:tt)*) => { $crate::utils::log::log_ok($indent, $crate::fmt!($($args)*)) };
	($($args:tt)*) => { $crate::ok!(0 => $($args)*) };
} pub use ok;

/// Log an error, failure or issue the user should be aware of.
#[macro_export] macro_rules! bad {
	($indent:expr => $($args:tt)*) => { $crate::utils::log::log_bad($indent, $crate::fmt!($($args)*)) };
	($($args:tt)*) => { $crate::bad!(0 => $($args)*) };
} pub use bad;

/// Log a generic informative message with no special colouring, which can be quickly skimmed past.
#[macro_export] macro_rules! debug {
	($indent:expr => $($args:tt)*) => { $crate::utils::log::log_debug($indent, $crate::fmt!($($args)*)) };
	($($args:tt)*) => { $crate::debug!(0 => $($args)*) };
} pub use debug;

/// Log a hint to the user that may help them fix an error.
#[macro_export] macro_rules! hint {
	($indent:expr => $($args:tt)*) => { $crate::utils::log::log_hint($indent, $crate::fmt!($($args)*)) };
	($($args:tt)*) => { $crate::hint!(0 => $($args)*) };
} pub use hint;


pub fn log_is   (indent: usize, msg: impl Display) { println!(" {:i$}{F}› {Y}{msg}", "", i = indent); }
pub fn log_ok   (indent: usize, msg: impl Display) { println!(" {:i$}{C}✓ {msg}",       "", i = indent); }
pub fn log_bad  (indent: usize, msg: impl Display) { println!(" {:i$}{R}× {msg}",       "", i = indent); }
pub fn log_debug(indent: usize, msg: impl Display) { println!(" {:i$}{F}│ {msg}",    "", i = indent); }
pub fn log_hint (indent: usize, msg: impl Display) { println!(" {:i$}{G}= hint: {msg}", "", i = indent); }

pub fn log_info(indent: usize, msg:  impl Display) {
	if !QUIET.get().is_some() {
		println!("{:i$} {F}› {msg}", "", i = indent);
	}
}


/// Print `err`, with surrounding line delimiters.
pub fn error(err: impl std::borrow::Borrow<SquarkError>)
{
	fn go(err: &SquarkError, mut indent: usize, mut already_shown_when: bool)
	{
		match err
		{
			SquarkError::Recoverable{ msg, hint, debug }
			| SquarkError::Unrecoverable{ msg, hint, debug }
			=> {
				bad!(indent => msg);
				
				for each in debug {
					if !each.is_empty() {
						debug!(indent => each);
					}
				}

				if !hint.is_empty() {
					hint!(indent => hint);
				}
			}

			SquarkError::Multiple{ when, errs } =>
			{
				if !already_shown_when && !when.is_empty() {
					already_shown_when = true;
					let count = err.len();
					let plural = if count == 1 {""} else {"s"};
					bad!(indent => "{count} error{plural} while {when}");
					line();
					indent += 3;
				}

				for (i, err) in errs.iter().enumerate() {
					if err.is_empty() {
						continue;
					}
					// 1st error already has preceeding line
					if i != 0 {
						if let SquarkError::Multiple{..} = &err {
							line();
						} else {
							// Don't print lines between lowest-level aggregated errors
							println!();
						}
					}
					go(err, indent, already_shown_when);
				}
			}

			SquarkError::External{ err, msg, backtrace } => {
				bad!(indent => msg);
				println!("\n   {R}{err:?}");
				println!("\n{backtrace}");
			}

			SquarkError::ABANDON => {
				#[cfg(test)]
				bad!("abandoned!");
			}
		}
	}

	if err.borrow().is_empty() {
		return;
	}

	line();
	go(err.borrow(), 0, false);
	line();
}
