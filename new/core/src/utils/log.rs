#![allow(unused)]

use super::colours::*;

use std::fmt::Display;


/// Log an action that Squarkdown is about to perform, to set up expectations.
#[macro_export] macro_rules! is {
	($var:ident)   => { $crate::utils::log::log_is(format!("{}", $var)) };
	($($args:tt)*) => { $crate::utils::log::log_is(format!($($args)*)) };
} pub use is;

/// Log a generic informative message with no special colouring, which can be quickly skimmed past.
#[macro_export] macro_rules! info {
	($var:ident)   => { $crate::utils::log::log_info(format!("{}", $var)) };
	($($args:tt)*) => { $crate::utils::log::log_info(format!($($args)*)) };
} pub use info;

/// Log a checkpoint that has successfully been reached.
#[macro_export] macro_rules! ok {
	($var:ident)   => { $crate::utils::log::log_ok(format!("{}", $var)) };
	($($args:tt)*) => { $crate::utils::log::log_ok(format!($($args)*)) };
} pub use ok;

/// Log an error, failure or issue the user should be aware of.
#[macro_export] macro_rules! bad {
	($var:ident)   => { $crate::utils::log::log_bad(format!("{}", $var)) };
	($($args:tt)*) => { $crate::utils::log::log_bad(format!($($args)*)) };
} pub use bad;


pub fn log_is(msg:   impl Display) { println!(" {}› {}{}", GREY, YELLOW, msg); }
pub fn log_info(msg: impl Display) { println!(" {}› {}",   GREY, msg); }
pub fn log_ok(msg:   impl Display) { println!(" {}✓ {}",   CYAN, msg); }
pub fn log_bad(msg:  impl Display) { println!(" {}× {}",   RED, msg); }
pub fn log_hint(msg: impl Display) { println!(" {}= {}",   GREEN, msg); }
