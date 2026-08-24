#![allow(unused)]

use std::fmt::Display;


const WHITE: &str = "\033[0m";
const GREY:  &str = "\033[90m";
const BLACK: &str = "\033[30m";

const RED:    &str = "\033[31m";
const GREEN:  &str = "\033[92m";
const YELLOW: &str = "\033[93m";
const BLUE:   &str = "\033[94m";
const PINK:   &str = "\033[95m";
const CYAN:   &str = "\033[96m";


/// Log Squarkdown's initial message on startup.
macro_rules! started {
	() => { log_started() };
}
pub(crate) use started;

/// Log an error, failure or issue the user should be aware of.
macro_rules! bad
{
	($msg:expr) => {
		$crate::utils::log::log_bad(format!("{}", $msg))
	};
	($msg:literal $(, $args:expr)* $(,)?) => {
		log_bad(format!($msg, $(, $args)*))
	};
}
pub(crate) use bad;


fn log_started()
{
	// FIXME
	println!("{PINK}Squarkdown v{}", "4.0");
	println!("{GREY}----------------");
	log_state("squarking up...");
}

pub(crate) fn log_state(msg: impl Display) { println!("  {}› {}{}", GREY, YELLOW, msg); }
pub(crate) fn log_info(msg: impl Display)  { println!("  {}› {}",   GREY, msg); }
pub(crate) fn log_good(msg: impl Display)  { println!("  {}✓ {}",   BLUE, msg); }
pub(crate) fn log_bad(msg: impl Display)   { println!("  {}× {}",   RED, msg); }
pub(crate) fn log_hint(msg: impl Display)  { println!("  {}= {}",   GREEN, msg); }
