pub type ParseResult<T = ()> = Result<T, ParseError>;


/// An error encountered while parsing the charm squark.
/// 
/// All except `NO_MATCH` are critical and will terminate the parser.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ParseError
{
	/// A speculative parse was unsuccessful, so the parser should fallback to something else.
	#[allow(non_camel_case_types)]
	NO_MATCH,

	/// The parser unexpectedly reached the end of its source.
	FatalEnd {
		/// What was the parser doing when it threw this error?
		origin: String,
	},

	/// Input did not match what was expected (required) by the context.
	UnexpectedInput {
		origin: String,
		expected: String,
		actual: String,
	}
}

impl std::error::Error for ParseError {}

impl std::fmt::Display for ParseError
{
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
	{
		match self {
			ParseError::FatalEnd { origin: cause }
				=> write!(f, "unexpected end of input while {cause}!"),
			
			ParseError::UnexpectedInput { origin, expected, actual } =>
				write!(f, "expected {expected}, but found {actual}, while {origin}"),
			
			_ => write!(f, "leaked internal error!")
		}
	}
}
