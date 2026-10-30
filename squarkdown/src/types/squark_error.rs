use crate::config::*;
use crate::utils::log;
use crate::utils::macros::*;

use std::borrow::Cow;


/// A large fallible operation in the Squarkdown pipeline which errors with [`SquarkError`].
/// 
/// Why the duplicated [`SquarkError`]? Well, Squarkdown doesn’t just have simple `Ok`/`Err`; it also needs to propagate [`SquarkError::Warning`]s and [`SquarkError::Recoverable`]s. So there are really 3 outcomes:
/// 
/// - `Ok(t, ..)` — success + no errors
/// - `Ok(t, e)` — success + errors (to be printed)
/// - `Err(e)` — failure + errors
/// 
/// Alternatively, you can think of `Err` as being reserved for conventional ‘exceptions’ in other languages, while `Ok(t, e)` aggregates errors to print.
/// 
/// Contrast with the lighter [`SquarkResult`].
pub type SquarkAction<T = ()> = Result<(T, SquarkError), SquarkError>;

impl From<SquarkError> for SquarkAction {
	fn from(errs: SquarkError) -> Self {
		Ok(((), errs))
	}
}

/// A small fallible operation which errors with [`SquarkError`].
/// 
/// Contrast with the heavier [`SquarkAction`].
pub type SquarkResult<T = ()> = Result<T, SquarkError>;


/// The global error type of [`SquarkResult`], used throughout the squarkup pipeline.
/// 
/// Squarkdown needs to handle errors in many different ways:
/// 
/// - [`Self::Unrecoverable`]: Critical errors that mean Squarkdown can’t continue properly inside a context, e.g. failing to find the project root.
/// - [`Self::Recoverable`]: Non-critical errors that Squarkdown can gracefully recover from.
/// - [`Self::Warning`]: Potentially unwanted behaviour that should still be flagged to the user.
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

	/// An issue that Squarkdown is flagging to the user.
	Warning {
		msg: String,
		hint: String,
		debug: Vec<String>,
	},

	/// A non-fatal error which Squarkdown can recover from. Handling depends on `config.errors.on_error`.
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
	/// Handling depends on `config.errors.on-error`.
	Multiple {
		when: Cow<'static, str>,
		errs: Vec<SquarkError>,
	},

	/// A fatal error that crashes Squarkdown, caused by external factors such as a file read failure.
	External {
		msg: String,
		err: Box<dyn std::error::Error + Send>,
		backtrace: std::backtrace::Backtrace,
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
			msg: str!("Unexpected external error"),
			err: Box::new(error),
			backtrace: std::backtrace::Backtrace::capture(),
		}
	}
}

impl SquarkError
{
	/// Is this a non-recoverable error that immediately crashes its parent context?
	/// 
	/// “Parent context” is either the entire Squarkdown pipeline (if this error occurs during initialisation) or processing for a singular page.
	/// 
	/// This includes:
	/// 
	/// - [`SquarkError::Unrecoverable`]
	/// 
	/// These errors are 'unrecoverable' because they invalidate how everything works down the line. For instance, a missing required field.
	/// 
	/// Other errors, like an invalid field, are 'recoverable' in that they don't break *everything*. For instance, they might only change how the output renders.
	/// 
	/// Contrast with [`is_fine`](Self::is_fine).
	#[must_use]
	pub fn is_fatal(&self) -> bool
	{
		match self
		{
			Self::Unrecoverable{..} | Self::External{..}
				=> true,

			Self::ABANDON | Self::Warning{..} | Self::Recoverable{..}
				=> false,

			Self::Multiple{ errs, .. }
				=> errs.iter().any(SquarkError::is_fatal),
		}
	}

	/// Is this an empty error that should be ignored?
	/// 
	/// This includes:
	/// 
	/// - [`SquarkError::Multiple`] error with no aggregated errors
	/// - [`SquarkError::ABANDON`] to skip an operation
	/// 
	/// Contrast with [`is_fatal`](Self::is_fatal).
	#[must_use]
	pub fn is_fine(&self) -> bool
	{
		matches!(self, Self::ABANDON)
		|| matches!(self, Self::Multiple{ errs, .. } if errs.iter().all(Self::is_fine))
	}

	/// Set the `hint` text of a [`SquarkError::Recoverable`] or [`SquarkError::Unrecoverable`].
	pub fn set_hint(&mut self, text: String) -> bool
	{
		if let Self::Recoverable{ hint, .. } | Self::Unrecoverable{ hint, .. } = self {
			*hint = text;
			true
		} else {
			false
		}
	}
}

/// Implementations specific to [`SquarkError::Multiple`]
impl SquarkError
{
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

	/// Count the number of atomic errors contained in this [`SquarkError`].
	/// 
	/// For [`SquarkError::Multiple`], this recursively counts its aggregated errors. For all other error variants, this returns `1`.
	pub fn len(&self) -> usize
	{
		if let Self::Multiple{ errs, ..} = self {
			errs.iter().map(Self::len).sum()
		} else {
			1
		}
	}

	/// Is this an empty [`SquarkError::Multiple`]?
	pub fn is_empty(&self) -> bool
	{
		self.len() == 0
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

			  Self::Warning{ msg, hint, .. }
			| Self::Recoverable{ msg, hint, .. }
			| Self::Unrecoverable{ msg, hint, .. }
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

impl IntoIterator for SquarkError
{
	type Item = SquarkError;
	type IntoIter = <Vec<SquarkError> as IntoIterator>::IntoIter;

	/// Iterate over the children of a [`SquarkError::Multiple`].
	fn into_iter(self) -> Self::IntoIter
	{
		if let Self::Multiple{ errs, .. } = self {
			errs.into_iter()
		} else {
			vec![self].into_iter()
		}
	}
}

impl std::fmt::Display for SquarkError
{
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
	{
		match self
		{
			Self::ABANDON => write!(f, "ABANDONED"),

			  Self::Warning{ msg, .. }
			| Self::Recoverable{ msg, .. }
			| Self::Unrecoverable{ msg, .. }
			| Self::External{ msg, .. }
				=> write!(f, "{msg}"),

			Self::Multiple{ errs, .. } => {
				for err in errs {
					writeln!(f, "{err}")?;
				}
				Ok(())
			}
		}
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

	/// Split an iterator of [`SquarkResult`]s into `(oks, errs)`.
	/// 
	/// `errs` is always a [`SquarkError::Multiple`].
	fn partition_squark(self, when: impl Into<Cow<'static, str>>) -> (Vec<T>, SquarkError)
	{
		let mut oks = vec![];
		let mut errs = SquarkError::multiple(when);

		for each in self {
			match each {
				Ok(v)  => { oks.push(v); }
				Err(e) => { errs.push(e); }
			}
		}

		(oks, errs)
	}
}

impl<I, T> CollectSquark<T> for I
	where I: Iterator<Item = SquarkResult<T>>
{}
