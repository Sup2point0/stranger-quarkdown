#![allow(unused)]

use super::colours::*;

use std::fmt::Display;


/// Log an action that Squarkdown is about to perform, to set up expectations.
#[macro_export] macro_rules! is {
	($msg:literal) => { $crate::log::log_is(fmt!($msg)) };
	($var:expr)    => { $crate::log::log_is(fmt!("{}", $var)) };
	($($args:tt)*) => { $crate::log::log_is(fmt!($($args)*)) };
} pub use is;

/// Log a generic informative message with no special colouring, which can be quickly skimmed past.
#[macro_export] macro_rules! info {
	($msg:literal) => { $crate::log::log_info(fmt!($msg)) };
	($var:expr)    => { $crate::log::log_info(fmt!("{}", $var)) };
	($($args:tt)*) => { $crate::log::log_info(fmt!($($args)*)) };
} pub use info;

/// Log a checkpoint that has successfully been reached.
#[macro_export] macro_rules! ok {
	($msg:literal) => { $crate::log::log_ok(fmt!($msg)) };
	($var:expr)    => { $crate::log::log_ok(fmt!("{}", $var)) };
	($($args:tt)*) => { $crate::log::log_ok(fmt!($($args)*)) };
} pub use ok;

/// Log an error, failure or issue the user should be aware of.
#[macro_export] macro_rules! bad {
	($msg:literal) => { $crate::log::log_bad(fmt!($msg)) };
	($var:expr)    => { $crate::log::log_bad(fmt!("{}", $var)) };
	($($args:tt)*) => { $crate::log::log_bad(fmt!($($args)*)) };
} pub use bad;

/// Log a hint to the user that may help them fix an error.
#[macro_export] macro_rules! hint {
	($msg:literal) => { $crate::log::log_hint(fmt!($msg)) };
	($var:expr)    => { $crate::log::log_hint(fmt!("{}", $var)) };
	($($args:tt)*) => { $crate::log::log_hint(fmt!($($args)*)) };
} pub use hint;


pub fn line() { println!("{GREY}────────────────────────"); }

pub fn log_is(msg:   impl Display) { println!(" {}› {}{}",     GREY, Y, msg); }
pub fn log_info(msg: impl Display) { println!(" {}› {}",       GREY, msg); }
pub fn log_ok(msg:   impl Display) { println!(" {}✓ {}",       C, msg); }
pub fn log_bad(msg:  impl Display) { println!(" {}× {}",       R, msg); }
pub fn log_hint(msg: impl Display) { println!(" {}= hint: {}", G, msg); }
