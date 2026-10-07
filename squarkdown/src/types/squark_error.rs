use crate::config::*;
use crate::log;
use crate::macros::*;

use std::borrow::Cow;


/// An operation which may error with a [`SquarkError`].
pub type SquarkResult<T = ()> = Result<T, SquarkError>;


/// The global error type of [`SquarkResult`], used throughout the squarkup pipeline.
/// 
/// Squarkdown needs to handle errors in many different ways:
/// 
/// - [`Self::Unrecoverable`]: Mission-critical errors. Squarkdown can't continue properly.
/// - [`Self::Recoverable`]: Undesirable, but localised, so Squarkdown can still recover from them.
/// - [`Self::Multiple`]: If Squarkdown encounters multiple errors, it aggregates them and presses on to do a best-effort job.
/// - [`Self::External`]: Squarkdown interfaces with many external APIs, which all return their own errors.
/// 
/// [`Self::ABANDON`] is also used for propagation when skipping operations.
/// 
/// How errors are handled depends on the user's `config.errors.on-error`.
#[derive(Debug)]
pub enum SquarkError
{
	/// The current operation can be abandoned.
	/// 
	/// This is used in the parser for speculative parsing, and elsewhere for skipping files.
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
		when: Cow<'static, str>,
		errs: Vec<SquarkError>,
	},

	/// A fatal error that crashes Squarkdown, caused by external factors such as a file read failure.
	External {
		msg: String,
		err: Box<dyn std::error::Error + Send>,
	},
}

/// Constructors
impl SquarkError
{
	/// Construct an empty [`Self::Multiple`] for aggregating errors.
	/// 
	/// Use alongside the [`catch`] macro.
	#[must_use]
	pub fn multiple(when: impl Into<Cow<'static, str>>) -> Self
	{
		Self::Multiple { when: when.into(), errs: vec![] }
	}

	/// Construct a [`Self::External`] with only a plain error message.
	#[must_use]
	pub fn external(error: impl std::error::Error + Send + 'static) -> Self
	{
		Self::External {
			msg: str!("unexpected external error"),
			err: Box::new(error),
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

			Self::Multiple{ errs, .. }
				=> errs.iter().any(SquarkError::is_fatal),
		}
	}

	pub fn len(&self) -> usize
	{
		if let Self::Multiple{ errs, ..} = self {
			errs.iter().map(Self::len).sum()
		} else {
			1
		}
	}

	/// Add an error to a [`Self::Multiple`] instance.
	pub fn push(&mut self, error: SquarkError) -> bool
	{
		if let Self::Multiple{ errs, .. } = self {
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
	/// Is this an empty error that should be ignored?
	/// 
	/// This includes:
	/// 
	/// - [`SquarkError::Multiple`] error with no aggregated errors
	/// - [`SquarkError::ABANDON`] to skip an operation
	#[must_use]
	pub fn is_fine(&self) -> bool
	{
		matches!(self, Self::ABANDON)
		||
		matches!(self, Self::Multiple{ errs, .. }
			if errs.iter().all(Self::is_fine)
		)
	}

	/// Propagate a [`SquarkError::Multiple`] if it is non-empty, otherwise return `t`.
	/// 
	/// ```ignore
	/// fn may_fail() -> SquarkResult<usize>
	/// {
	///    let errs = SquarkError::multiple("example");
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
	/// ```ignore
	/// fn may_fail() -> SquarkResult<String>
	/// {
	///    let errs = SquarkError::multiple("example");
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
		if self.is_fine() {
			Ok(f())
		} else {
			Err(self)
		}
	}

	/// Propagate a non-empty [`SquarkError`] depending on `config`.
	/// 
	/// ```ignore
	/// fn may_fail(config: &SquarkupConfig) -> SquarkResult
	/// {
	///    let err = try_something();
	/// 
	///    // For a recoverable error:
	///    // - If `config.errors.on-error` is `KILL`, this returns `Err(err)`.
	///    // - If `config.errors.on-error` is `WARN`, this prints the error and returns `Ok(2)`.
	///    err.depends(config)?;
	///    Ok(2)
	/// }
	/// ```
	pub fn depends(self, config: &SquarkupConfig) -> SquarkResult
	{
		self.or_depends((), config)
	}

	/// Propagate a non-empty [`SquarkError`] depending on `config`, otherwise return `t`.
	/// 
	/// ```ignore
	/// fn may_fail(config: &SquarkupConfig) -> SquarkResult<usize>
	/// {
	///    let err = SquarkError::Recoverable {..};
	/// 
	///    // For a recoverable error:
	///    // - If `config.errors.on-error` is `KILL`, this returns `Err(err)`.
	///    // - If `config.errors.on-error` is `WARN`, this prints the error and returns `Ok(2)`.
	///    err.or_depends(2, config)?;
	///    Ok(2)
	/// }
	/// ```
	pub fn or_depends<T>(self, t: T, config: &SquarkupConfig) -> SquarkResult<T>
	{
		if self.is_fine() {
			Ok(t)
		}
		else if self.is_fatal() || config.errors.on_error == ErrorAction::KILL {
			Err(self)
		}
		else {
			log::error(self);
			Ok(t)
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

			Self::Multiple { when, errs }
				=> when.contains(pat) || errs.iter().any(|err| err.contains(pat)),
		}
	}
}

impl<E> From<E> for SquarkError
	where E: std::error::Error + Send + 'static
{
	fn from(error: E) -> Self {
		Self::external(error)
	}
}


pub trait CollectSquark<T>: Iterator<Item = SquarkResult<T>> + Sized
{
	/// Collapse an iterator of [`SquarkResult`]s into a single [`SquarkResult`].
	fn collect_squark(self, when: impl Into<Cow<'static, str>>) -> SquarkResult<Vec<T>>
	{
		let mut vals = vec![];
		let mut errs = SquarkError::multiple(when);

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
