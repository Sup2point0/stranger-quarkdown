use crate::{
	colours::*,
	macros::*,
};

pub type SquarkResult<T = ()> = Result<T, SquarkError>;


#[derive(Debug, thiserror::Error)]
pub enum SquarkError
{
	/// A non-fatal error.
	/// 
	/// Handling depends on `config.errors.on_error`.
	#[error("{R}{msg}")]
	Recoverable {
		msg: String,
		hint: String,
		debug: Vec<String>,
	},

	/// A fatal error that crashes Squarkdown, irrespective of `config.errors.on_error`.
	#[error("{R}{msg}")]
	Unrecoverable {
		msg: String,
		hint: String,
		debug: Vec<String>,
	},

	/// Multiple errors, aggregated from an atomic operation.
	/// 
	/// Handling depends on `config::errors::on_error`.
	#[error("{R}multiple fatal errors")]
	Multiple {
		errs: Vec<SquarkError>,
	},

	/// A fatal error that crashes Squarkdown, caused by external factors such as a file read failure.
	#[error("{R}{msg}")]
	External {
		err: Box<dyn std::error::Error>,
		msg: String,
	},
}

/// Constructors
impl SquarkError
{
	/// Construct a [`Self::Unrecoverable`] with only a plain error message.
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
	pub fn multiple() -> Self
	{
		Self::Multiple { errs: vec![] }
	}

	/// Construct a [`Self::External`] with only a plain error message.
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
	pub fn is_fatal(&self) -> bool
	{
		match self
		{
			Self::Recoverable{..} => false,

			Self::Unrecoverable{..}
			| Self::External{..} => true,
			
			Self::Multiple{ errs } => errs.iter().any(|err| err.is_fatal()),
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
	pub fn is_empty(&self) -> bool
	{
		if let Self::Multiple { errs } = self && errs.is_empty() {
			true
		} else {
			false
		}
	}

	pub fn or<T>(self, t: T) -> SquarkResult<T>
	{
		self.or_else(|| t)
	}

	pub fn or_else<T>(self, f: impl FnOnce() -> T) -> SquarkResult<T>
	{
		if self.is_empty() {
			Ok(f())
		} else {
			Err(self)
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
