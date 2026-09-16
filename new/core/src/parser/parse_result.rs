pub enum ParseResult
{
	/// The parser successfully consumed input.
	ADVANCE,

	/// The parser failed to match a speculative parse and backtracked.
	BACKTRACK,
}
