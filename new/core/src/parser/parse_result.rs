pub type ParseResult<T = ()> = Result<T, ParseFailure>;

/// Indicates that a function only errors with [`ParseError::NO_MATCH`].
pub type Recoverable = ParseResult;


/// A possible error that should be propagated.
/// 
/// `NO_MATCH` and `DONE` are non-critical status indicators for short-circuiting. The rest are critical errors that terminate the parser.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ParseFailure
{
	/// A speculative parse was unsuccessful, so the parser should fallback to something else.
	#[allow(non_camel_case_types)]
	NO_MATCH,

	/// The parser unexpectedly reached the end of its source.
	FatalEnd {
		/// What was the parser doing when it threw this error?
		when: String,
	},

	/// The parser did not find input it expected (required).
	MissingInput {
		when: String,
		expected: String,
		actual: String,
	},

	/// Input did not match what was expected (required) by the context.
	UnexpectedInput {
		when: String,
		expected: String,
		actual: String,
	},

	IllegalInput {
		when: String,
		because: String,
		found: String,
	}
}

impl std::error::Error for ParseFailure {}

impl std::fmt::Display for ParseFailure
{
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
	{
		match self
		{
			Self::FatalEnd { when: cause }
				=> write!(f, "unexpected end of input while {cause}!"),
			
			Self::UnexpectedInput { when: origin, expected, actual } =>
				write!(f, "while {origin}: expected {expected}, but found {actual}"),
			
			_ => write!(f, "leaked internal error!")
		}
	}
}
