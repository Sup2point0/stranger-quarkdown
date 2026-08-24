#![allow(unused)]

use std::fmt::Display;


const WHITE: &str = "\x1b[0m";
const GREY:  &str = "\x1b[90m";
const BLACK: &str = "\x1b[30m";

const RED:    &str = "\x1b[31m";
const GREEN:  &str = "\x1b[92m";
const YELLOW: &str = "\x1b[93m";
const BLUE:   &str = "\x1b[94m";
const PINK:   &str = "\x1b[95m";
const CYAN:   &str = "\x1b[96m";


/// Log Squarkdown's initial message on startup.
#[macro_export]
macro_rules! started {
	() => { log_started() };
}
pub use started;

/// Log a checkpoint that has successfully been reached.
#[macro_export] macro_rules! ok {
	($msg:expr)                           => { $crate::utils::log::log_ok(format!("{}", $msg)) };
	($msg:literal $(, $args:expr)* $(,)?) => { log_ok(format!($msg, $(, $args)*)) };
}
pub use ok;

/// Log an error, failure or issue the user should be aware of.
#[macro_export] macro_rules! bad {
	($msg:expr)                           => { $crate::utils::log::log_bad(format!("{}", $msg)) };
	($msg:literal $(, $args:expr)* $(,)?) => { log_bad(format!($msg, $(, $args)*)) };
}
pub use bad;


fn log_started()
{
	// FIXME
	println!("{PINK}Squarkdown v{}", "4.0");
	println!("{GREY}----------------");
	log_state("squarking up...");
}

pub fn log_state(msg: impl Display) { println!("  {}› {}{}", GREY, YELLOW, msg); }
pub fn log_info(msg: impl Display)  { println!("  {}› {}",   GREY, msg); }
pub fn log_ok(msg: impl Display)  { println!("  {}✓ {}",   BLUE, msg); }
pub fn log_bad(msg: impl Display)   { println!("  {}× {}",   RED, msg); }
pub fn log_hint(msg: impl Display)  { println!("  {}= {}",   GREEN, msg); }
