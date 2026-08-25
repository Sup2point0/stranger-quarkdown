use crate::utils::macros::*;

pub type SquarkResult<T = ()> = Result<T, SquarkError>;


#[derive(Debug, thiserror::Error)]
pub enum SquarkError
{
	/// A non-fatal error.
	/// 
	/// Handling depends on `config.errors.on_error`.
	#[error("{msg}")]
	Recoverable {
		msg: String,
	},

	/// Multiple non-fatal errors, aggregated from an atomic operation.
	/// 
	/// Handling depends on `config.errors.on_error`.
	#[error("many errors")]
	ManyRecoverable {
		errs: Vec<SquarkError>,
	},

	/// A fatal error that crashes Squarkdown.
	#[error("{msg}")]
	Unrecoverable {
		msg: String,
		hint: String,
		debug: Vec<String>,
	},

	/// A fatal error that crashes Squarkdown, caused by external factors such as a file read failure.
	#[error("{0}")]
	External(Box<dyn std::error::Error>),
}

impl SquarkError
{
	/// Construct a `SquarkError::Unrecoverable` with only a plain error message.
	pub fn fatal(msg: &str) -> Self
	{
		Self::Unrecoverable {
			msg: msg.to_owned(),
			hint: str!(),
			debug: vec![],
		}
	}

	pub fn external(e: impl std::error::Error + 'static) -> Self
	{
		Self::External(Box::new(e))
	}
}
