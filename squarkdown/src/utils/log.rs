//! This module implements Squarkdown’s logging functionality.
//! 
//! This consists primarily of macros along with some functions:
//! 
//! - The single-line `log::*!`, such as [`log::is!`](is) or [`log::ok!`](ok) for printing small messages.
//! - The multi-line [`log::error()`](error) for printing large error messages.
//! - [`log::line()`](line) for printing nice line separators.
//! 
//! To prevent interleaved console output, logging errors is made atomic via [`LOG_LOCK`].

use crate::prelude::*;
use crate::cli::QUIET;
use crate::utils::colours::*;

use std::borrow::{ Borrow };
use std::fmt::{ Display };
use std::io::{ self, Write };
use std::sync::{ Mutex };


/// Global lock for logging to allow multi-line messages to be printed atomically without interleaving from multithreading.
/// 
/// Note that this only protects [`log::error()`](error) calls from each other, not other calls like [`log::info!()`](info), unless they are wrapped in [`log_lock!`].
pub static LOG_LOCK: Mutex<()> = Mutex::new(());

/// Did the last printed message have a trailing [`log::line()`](line) printed?
/// 
/// This is used to avoid printing 2 lines one after the other in the console, which looks funky...
static LAST_LOG_HAD_TRAILING_LINE: Mutex<bool> = Mutex::new(false);


/// # `log-lock! { ... }`
/// 
/// Wrap the given block in a lock on [`LOG_LOCK`] so that it prints atomically.
/// 
/// ```ignore
/// log_lock! {
///   log::info!("This log will be atomic.");
/// }
/// ```
#[macro_export]
macro_rules! lock {
	($($body:tt)+) => {
		{
			let _lock = $crate::utils::log::LOG_LOCK.lock().expect("global log lock was poisoned!!");
			$($body)+
		}
	}
}
pub use lock;


/// Print a line to the console.
pub fn line() { println!("{F}{}", "─".repeat(69)); }

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

/// Log an error or failure.
#[macro_export] macro_rules! bad {
	($indent:expr => $($args:tt)*) => { $crate::utils::log::log_bad($indent, $crate::fmt!($($args)*)) };
	($($args:tt)*) => { $crate::bad!(0 => $($args)*) };
} pub use bad;

/* NOTE: Would like to use `warn!` but this clashes with prelude */
/// Log a non-critical issue the user should be aware of.
#[macro_export] macro_rules! warns {
	($indent:expr => $($args:tt)*) => { $crate::utils::log::log_warn($indent, $crate::fmt!($($args)*)) };
	($($args:tt)*) => { $crate::warns!(0 => $($args)*) };
} pub use warns;

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


macro_rules! impl_log {
	(
		$ident:ident => $($prints:tt)+
	) => {
		pub fn $ident(indent: usize, msg: impl Display)
		{
			let mut l = LAST_LOG_HAD_TRAILING_LINE.lock().expect("log trailing line lock was poisoned!!");
			println!($($prints)+, "", msg, indent = indent);
			*l = true;
		}
	}
}

impl_log! { log_is    => " {:indent$}{F}› {W}{}" }
impl_log! { log_ok    => " {:indent$}{C}✓ {}" }
impl_log! { log_bad   => " {:indent$}{R}× {}" }
impl_log! { log_warn  => " {:indent$}{Y}▲ {}" }
impl_log! { log_debug => " {:indent$}{F}│ {}" }
impl_log! { log_hint  => " {:indent$}{G}= hint: {}" }

pub fn log_info(indent: usize, msg: impl Display) {
	if !QUIET.get().is_some() {
		println!("{:indent$} {F}› {msg}", "", indent = indent);
	}
}


/// Print `err`, with surrounding line delimiters.
/// 
/// This is atomic to prevent interleaving from multiple threads calling [`error()`] at once.
pub fn error(err: impl Borrow<SquarkError>)
{
	if err.borrow().is_empty() {
		return;
	}

	let _lock = LOG_LOCK.lock()
		.expect("global log lock was poisoned!!");

	/* NOTE: Duplicate line avoidance is best-effort: we don’t want to slow down the entire pipeline by waiting on locks _just_ for nicer formatting... */
	match LAST_LOG_HAD_TRAILING_LINE.try_lock() {
		Ok(v) => if !*v { line() },
		Err(..) => line(),
	}

	print_error(err.borrow(), 0, false);

	line();

	*LAST_LOG_HAD_TRAILING_LINE.lock().expect("log trailing line lock was poisoned!!")
		= true;

	let _ = io::stdout().flush();
}

fn print_error(err: &SquarkError, mut indent: usize, mut already_shown_when: bool)
{
	match err
	{
		  SquarkError::Warning{ msg, hint, debug }
		| SquarkError::Recoverable{ msg, hint, debug }
		| SquarkError::Unrecoverable{ msg, hint, debug }
		=> {
			if let SquarkError::Warning{..} = err {
				warns!(indent => msg);
			} else {
				bad!(indent => msg);
			}
			
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
				indent += 2;
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
				print_error(err, indent, already_shown_when);
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
