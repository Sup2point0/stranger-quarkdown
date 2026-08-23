pub type ParseResult<T = ()> = Result<T, ParseError>;


#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ParseError
{
	EndOfFile,
	NoMatch,
	FatalEnd,
}

impl std::error::Error for ParseError {}

impl std::fmt::Display for ParseError
{
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
	{
		unimplemented!()
	}
}
