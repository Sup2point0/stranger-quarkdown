#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ctx {
	MARKDOWN,
	CODE_INLINE,
	CODE_BLOCK,
	LINK,
	COMMENT,
	SQUARK_LEAVE,
	SQUARK_SLASH,
	SQUARK_ONLY,
}


#[derive(Debug)]
pub struct ContextStack
{
	stack: Vec<Ctx>,
}

/// Constructors
impl ContextStack
{
	pub fn new() -> Self {
		Self {
			stack: vec![],
		}
	}
}

/// Implementation
impl ContextStack
{
	pub fn stack(&self) -> &[Ctx]
	{
		&self.stack
	}

	/// What's the current context?
	pub fn current(&self) -> Ctx {
		*self.stack.last().unwrap_or(&Ctx::MARKDOWN)
	}

	pub fn push(&mut self, ctx: Ctx) {
		self.stack.push(ctx);
	}

	pub fn pop(&mut self, ctx: Ctx)
	{
		if self.current() == ctx {
			self.stack.pop();
		}
	}

	/// Pop `ctx` from the stack as deep as possible, regardless of the current context.
	/// 
	/// For instance, when seeing a `-->`, this should completely terminate a comment context, which might look like:
	/// 
	/// ```ts
	/// [COMMENT]       // normal
	/// [COMMENT, ...]  // unclosed, but we don't care cuz it's a comment
	/// [COMMENT, COMMENT]  // user used <!-- <!--
	/// ```
	/// 
	/// If `ctx` is not present in the context stack, this is a no-op.
	pub fn force_pop(&mut self, ctx: Ctx)
	{
		if let Some(idx) = self.stack.iter().rposition(|c| *c == ctx) {
			self.stack.truncate(idx);
		}

		while self.current() == ctx {
			self.stack.pop();
		}
	}
}


#[cfg(test)]
mod test
{
	use super::*;
	
	#[test] fn force_pop_easy()
	{
		let mut ctx = ContextStack::new();

		ctx.push(Ctx::CODE_BLOCK);

		ctx.push(Ctx::COMMENT);
		ctx.push(Ctx::LINK);
		ctx.force_pop(Ctx::COMMENT);
		assert_eq!( ctx.current(), Ctx::CODE_BLOCK );
	}

	#[test] fn force_pop_medium()
	{
		let mut ctx = ContextStack::new();
		ctx.push(Ctx::CODE_BLOCK);

		ctx.push(Ctx::COMMENT);
		ctx.push(Ctx::LINK);
		ctx.push(Ctx::COMMENT);
		ctx.force_pop(Ctx::COMMENT);
		assert_eq!( ctx.current(), Ctx::LINK );
		
		ctx.force_pop(Ctx::COMMENT);
		assert_eq!( ctx.current(), Ctx::CODE_BLOCK );
	}

	#[test] fn force_pop_hard()
	{
		let mut ctx = ContextStack::new();
		ctx.push(Ctx::CODE_BLOCK);

		ctx.push(Ctx::COMMENT);
		ctx.push(Ctx::LINK);
		ctx.push(Ctx::CODE_INLINE);

		ctx.force_pop(Ctx::CODE_BLOCK);
		assert_eq!( ctx.current(), Ctx::MARKDOWN );
	}
}
