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
		errors: Vec<SquarkError>,
	},

	/// A fatal error that crashes Squarkdown.
	#[error("{msg}")]
	Unrecoverable {
		msg: String,
		debug: Vec<String>,
	},

	/// A fatal error that crashes Squarkdown, caused by external factors such as a file read failure.
	#[error("{0}")]
	External(Box<dyn std::error::Error>),
}
