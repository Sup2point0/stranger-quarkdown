#![allow(unused)]

use crate::prelude::*;
use crate::colours::*;

use std::fmt::Display;


/// Log an action that Squarkdown is about to perform, to set up expectations.
#[macro_export] macro_rules! is {
	($msg:literal) => { $crate::log::log_is(format_args!($msg)) };
	($var:expr)    => { $crate::log::log_is(format_args!("{}", $var)) };
	($($args:tt)*) => { $crate::log::log_is(format_args!($($args)*)) };
} pub use is;

/// Log a generic informative message with no special colouring, which can be quickly skimmed past.
#[macro_export] macro_rules! info {
	($msg:literal) => { $crate::log::log_info(format_args!($msg)) };
	($var:expr)    => { $crate::log::log_info(format_args!("{}", $var)) };
	($($args:tt)*) => { $crate::log::log_info(format_args!($($args)*)) };
} pub use info;

/// Log a checkpoint that has successfully been reached.
#[macro_export] macro_rules! ok {
	($msg:literal) => { $crate::log::log_ok(format_args!($msg)) };
	($var:expr)    => { $crate::log::log_ok(format_args!("{}", $var)) };
	($($args:tt)*) => { $crate::log::log_ok(format_args!($($args)*)) };
} pub use ok;

/// Log an error, failure or issue the user should be aware of.
#[macro_export] macro_rules! bad {
	($msg:literal) => { $crate::log::log_bad(format_args!($msg)) };
	($var:expr)    => { $crate::log::log_bad(format_args!("{}", $var)) };
	($($args:tt)*) => { $crate::log::log_bad(format_args!($($args)*)) };
} pub use bad;

/// Log a hint to the user that may help them fix an error.
#[macro_export] macro_rules! hint {
	($msg:literal) => { $crate::log::log_hint(format_args!($msg)) };
	($var:expr)    => { $crate::log::log_hint(format_args!("{}", $var)) };
	($($args:tt)*) => { $crate::log::log_hint(format_args!($($args)*)) };
} pub use hint;


pub fn line() { println!("{GREY}────────────────────────"); }

pub fn log_is(msg:   impl Display) { println!(" {}› {}{}",     GREY, Y, msg); }
pub fn log_info(msg: impl Display) { println!(" {}› {}",       GREY, msg); }
pub fn log_ok(msg:   impl Display) { println!(" {}✓ {}",       C, msg); }
pub fn log_bad(msg:  impl Display) { println!(" {}× {}",       R, msg); }
pub fn log_hint(msg: impl Display) { println!(" {}= hint: {}", G, msg); }


/// Print `err`, with surrounding line delimiters.
pub fn error(err: SquarkError)
{
	match err
	{
		SquarkError::Recoverable{ msg, hint, debug }
		| SquarkError::Unrecoverable{ msg, hint, debug }
		=> {
			bad!(msg);
			
			for each in debug {
				info!(each);
			}

			if !hint.is_empty() {
				hint!(hint);
			}
		}
		SquarkError::Multiple{ errs } => {
			for (i, err) in errs.into_iter().enumerate() {
				if i != 0 { line(); }
				error(err);
			}
		}
		SquarkError::External{ err, msg } => {
			bad!(msg);
			line();
			println!("{R}{err:?}");
		}
		SquarkError::ABANDON => (),
	}
}
