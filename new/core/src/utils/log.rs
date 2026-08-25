#![allow(unused)]

use super::colours::*;

use std::fmt::Display;


/// Log an action that Squarkdown is about to perform, to set up expectations.
#[macro_export] macro_rules! is {
	($msg:literal) => { $crate::utils::log::log_is(format!($msg)) };
	($var:expr)    => { $crate::utils::log::log_is(format!("{}", $var)) };
	($($args:tt)*) => { $crate::utils::log::log_is(format!($($args)*)) };
} pub use is;

/// Log a generic informative message with no special colouring, which can be quickly skimmed past.
#[macro_export] macro_rules! info {
	($msg:literal) => { $crate::utils::log::log_info(format!($msg)) };
	($var:expr)    => { $crate::utils::log::log_info(format!("{}", $var)) };
	($($args:tt)*) => { $crate::utils::log::log_info(format!($($args)*)) };
} pub use info;

/// Log a checkpoint that has successfully been reached.
#[macro_export] macro_rules! ok {
	($msg:literal) => { $crate::utils::log::log_ok(format!($msg)) };
	($var:expr)    => { $crate::utils::log::log_ok(format!("{}", $var)) };
	($($args:tt)*) => { $crate::utils::log::log_ok(format!($($args)*)) };
} pub use ok;

/// Log an error, failure or issue the user should be aware of.
#[macro_export] macro_rules! bad {
	($msg:literal) => { $crate::utils::log::log_bad(format!($msg)) };
	($var:expr)    => { $crate::utils::log::log_bad(format!("{}", $var)) };
	($($args:tt)*) => { $crate::utils::log::log_bad(format!($($args)*)) };
} pub use bad;

/// Log a hint to the user that may help them fix an error.
#[macro_export] macro_rules! hint {
	($msg:literal) => { $crate::utils::log::log_hint(format!($msg)) };
	($var:expr)    => { $crate::utils::log::log_hint(format!("{}", $var)) };
	($($args:tt)*) => { $crate::utils::log::log_hint(format!($($args)*)) };
} pub use hint;

/// Log an *important* path that has successfully been resolved, using `log::ok!()`.
#[macro_export] macro_rules! found {
	($msg:literal, $path:expr) => {
		if let Some(normalised) = path_slash::PathBufExt::to_slash(&$path) {
			$crate::utils::log::log_ok(format!($msg, normalised))
		} else {
			$crate::utils::log::log_ok(format!($msg, $path.display()))
		}
	};
} pub use found;

/// Log a generic path that has successfully been resolved, using `log::info!()`.
#[macro_export] macro_rules! info_path {
	($msg:literal, $path:expr) => {
		if let Some(normalised) = path_slash::PathBufExt::to_slash(&$path) {
			$crate::utils::log::log_info(format!($msg, normalised))
		} else {
			$crate::utils::log::log_info(format!($msg, $path.display()))
		}
	};
} pub use info_path;


pub fn line() { println!("{GREY}────────────────────────"); }

pub fn log_is(msg:   impl Display) { println!(" {}› {}{}", GREY, YELLOW, msg); }
pub fn log_info(msg: impl Display) { println!(" {}› {}",   GREY, msg); }
pub fn log_ok(msg:   impl Display) { println!(" {}✓ {}",   CYAN, msg); }
pub fn log_bad(msg:  impl Display) { println!(" {}× {}",   RED, msg); }
pub fn log_hint(msg: impl Display) { println!(" {}= hint: {}",   GREEN, msg); }
