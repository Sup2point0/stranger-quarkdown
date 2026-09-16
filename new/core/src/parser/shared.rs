pub enum ParseResult
{
	/// The parser successfully consumed input.
	ADVANCE,

	/// The parser failed to match a speculative parse and backtracked.
	BACKTRACK,
}


macro_rules! ctx
{
	($self:ident, $ctx:path => $body:block) => {
		{
			$self.ctx.push($ctx);
			let r = { $body };
			$self.ctx.force_pop($ctx);
			r
		}
	};
}
pub(super) use ctx;
