pub type ParseResult<T = ()> = Result<T, ParseError>;


// TODO split properly

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ParseError
{
	// == NON-CRITICAL == //

	/// The required input for a parse wasn't present.
	NoMatch,

	/// This file does not have a `<!-- #SQUARK live! -->` so does not need to be squarked up.
	NotLive,


	// == CRITICAL == //

	/// The parser unexpectedly reached the end of its source.
	FatalEnd {
		/// What was the parser doing when it threw this error?
		cause: String,
	},

	/// Input did not match what was expected (required) by the context.
	MissingInput {
		expected: &'static str,
		actual: String,
	}
}

impl std::error::Error for ParseError {}

impl std::fmt::Display for ParseError
{
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
	{
		match self {
			ParseError::FatalEnd { cause }
				=> write!(f, "Unexpected end of input while {cause}!"),
			
			ParseError::MissingInput { expected, actual } =>
				write!(f, "Expected {expected}, but found {actual}"),
			
			_ => write!(f, "Leaked internal error!")
		}
	}
}
