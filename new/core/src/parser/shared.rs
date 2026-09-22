macro_rules! ctx
{
	($self:ident, $ctx:expr => $body:block) => {
		{
			$self.ctx.push($ctx);
			let r = { $body };
			$self.ctx.try_pop($ctx)?;
			r
		}
	};
}
pub(super) use ctx;
