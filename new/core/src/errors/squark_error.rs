use crate::{
	macros::*,
};

pub type SquarkResult<T = ()> = Result<T, SquarkError>;


/// The global error type used throughout the squarkup pipeline.
/// 
/// Squarkdown needs to handle errors in many different ways:
/// 
/// - Some errors are mission-critical, and Squarkdown can't continue properly if it encounters them.
/// - Some errors are undesirable, but localised, so Squarkdown can still recover from them.
/// - If Squarkdown encounters multiple errors, it aggregates them and presses on to do a best-effort job.
/// - Squarkdown interfaces with many external APIs, which all return their own errors.
/// 
/// How errors are handled depends on the user's `config.errors.on-error`.
#[derive(Debug)]
pub enum SquarkError
{
	/// The current operation can be abandoned.
	/// 
	/// Currently only used in the parser for speculative parsing.
	ABANDON,

	/// A non-fatal error which Squarkdown can recover from, sorta like a warning.
	/// 
	/// Handling depends on `config.errors.on_error`.
	Recoverable {
		msg: String,
		hint: String,
		debug: Vec<String>,
	},

	/// A fatal error that crashes Squarkdown, irrespective of `config.errors.on_error`, sorta like a panic.
	Unrecoverable {
		msg: String,
		hint: String,
		debug: Vec<String>,
	},

	/// Multiple errors, aggregated from an atomic operation.
	/// 
	/// Handling depends on `config::errors::on_error`.
	Multiple {
		errs: Vec<SquarkError>,
	},

	/// A fatal error that crashes Squarkdown, caused by external factors such as a file read failure.
	External {
		err: Box<dyn std::error::Error>,
		msg: String,
	},
}

/// Constructors
impl SquarkError
{
	/// Construct a [`Self::Unrecoverable`] with only a plain error message.
	#[must_use]
	pub fn fatal(msg: &str) -> Self
	{
		Self::Unrecoverable {
			msg: msg.to_owned(),
			hint: str!(),
			debug: vec![],
		}
	}

	/// Construct an empty [`Self::Multiple`] for aggregating errors.
	/// 
	/// Use alongside the [`catch`] macro.
	#[must_use]
	pub fn multiple() -> Self
	{
		Self::Multiple { errs: vec![] }
	}

	/// Construct a [`Self::External`] with only a plain error message.
	#[must_use]
	pub fn external(e: impl std::error::Error + 'static) -> Self
	{
		Self::External {
			err: bx!(e),
			msg: str!("unexpected external error"),
		}
	}
}

impl SquarkError
{
	/// Is this a non-recoverable error?
	/// 
	/// Some errors, like an invalid field, are 'recoverable' in that they don't break *everything*. For instance, they might only change how the output renders.
	/// 
	/// Other errors are 'unrecoverable' because they invalidate how everything works down the line. For instance, a missing required field.
	#[must_use]
	pub fn is_fatal(&self) -> bool
	{
		match self
		{
			Self::ABANDON | Self::Recoverable{..}
				=> false,

			Self::Unrecoverable{..} | Self::External{..}
				=> true,

			Self::Multiple{ errs }
				=> errs.iter().any(SquarkError::is_fatal),
		}
	}

	/// Add an error to a [`Self::Multiple`] instance.
	pub fn push(&mut self, error: SquarkError) -> bool
	{
		if let Self::Multiple{ errs } = self {
			errs.push(error);
			true
		} else {
			false
		}
	}
}

/// Implementations specific to [`SquarkError::Multiple`].
impl SquarkError
{
	/// Is this a [`SquarkError::Multiple`] error with no aggregated errors?
	#[must_use]
	pub fn is_empty(&self) -> bool
	{
		if let Self::Multiple { errs } = self && errs.is_empty() {
			true
		} else {
			false
		}
	}

	/// Propagate a [`SquarkError::Multiple`] if it is non-empty, otherwise return `t`.
	/// 
	/// ```rust
	/// # use squarkdown::core::*;
	/// fn may_fail() -> SquarkResult<usize>
	/// {
	///    let errs = SquarkError::multiple();
	/// 
	///    // If an error were present, `Err(errs)` is returned.
	///    // errs.push(SquarkError::...)
	/// 
	///    // With no aggregated errors, `Ok(1)` is returned.
	///    errs.or(1)
	/// }
	/// ```
	pub fn or<T>(self, t: T) -> SquarkResult<T>
	{
		self.or_else(|| t)
	}

	/// Propagate a [`SquarkError::Multiple`] if it is non-empty, otherwise return the lazily evaluated `f`.
	/// 
	/// ```rust
	/// # use squarkdown::core::*;
	/// fn may_fail() -> SquarkResult<String>
	/// {
	///    let errs = SquarkError::multiple();
	/// 
	///    // If an error were present, `Err(errs)` is returned.
	///    // errs.push(SquarkError::...)
	/// 
	///    // With no aggregated errors, `Ok("sup")` is returned.
	///    errs.or_else(|| "sup".to_string())
	/// }
	/// ```
	pub fn or_else<T>(self, f: impl FnOnce() -> T) -> SquarkResult<T>
	{
		if self.is_empty() {
			Ok(f())
		} else {
			Err(self)
		}
	}

	#[cfg(test)]
	pub fn contains(&self, pat: &str) -> bool
	{
		match self
		{
			Self::ABANDON | Self::External{..} => false,

			Self::Recoverable{ msg, hint, .. } | Self::Unrecoverable{ msg, hint, .. }
				=> msg.contains(pat) || hint.contains(pat),

			Self::Multiple { errs }
				=> errs.iter().any(|err| err.contains(pat)),
		}
	}
}


macro_rules! impl_from_error {
	($error_type:path) =>
	{
		impl From<$error_type> for SquarkError {
			fn from(e: $error_type) -> Self {
				Self::external(e)
			}
		}
	}
}

impl_from_error!(std::io::Error);


pub trait CollectSquark<T>: Iterator<Item = SquarkResult<T>> + Sized
{
	fn collect_squark(self) -> SquarkResult<Vec<T>>
	{
		let mut vals = vec![];
		let mut errs = SquarkError::multiple();

		for each in self {
			match each {
				Ok(val)  => { vals.push(val); }
				Err(err) => { errs.push(err); }
			}
		}

		errs.or(vals)
	}
}

impl<I, T> CollectSquark<T> for I
	where I: Iterator<Item = SquarkResult<T>>
{}
