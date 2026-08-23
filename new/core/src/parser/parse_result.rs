pub type ParseResult<T = ()> = Result<T, ParseError>;


#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ParseError
{
	/// The parser unexpectedly reached the end of its source.
	FatalEnd,

	/// The required input for a parse wasn't present.
	NoMatch,

	/// This file does not have a `<!-- #SQUARK live! -->` so does not need to be squarked up.
	NotLive,
}

impl std::error::Error for ParseError {}

impl std::fmt::Display for ParseError
{
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
	{
		unimplemented!()
	}
}
