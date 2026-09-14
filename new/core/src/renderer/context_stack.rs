#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Ctx {
	MARKDOWN,
	CODE,
	LEAVE { key: Option<String> },
	SLASH { key: Option<String> },
	ONLY,
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
	pub fn current(&self) -> &Ctx {
		self.stack.last().unwrap_or(&Ctx::MARKDOWN)
	}

	pub fn push(&mut self, ctx: Ctx) {
		self.stack.push(ctx);
	}

	/// Pop `ctx` from the stack if it is the currently active context, taking keys into account.
	pub fn try_pop(&mut self, ctx: Ctx) -> bool
	{
		if *self.current() == ctx {
			self.stack.pop();
			true
		} else {
			false
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

		while *self.current() == ctx {
			self.stack.pop();
		}
	}

	/// Is the current context `Ctx::LEAVE`?
	pub fn is_leave(&self) -> bool {
		matches!(self.current(), Ctx::LEAVE{..})
	}

	/// Is the current context `Ctx::SLASH`?
	pub fn is_slash(&self) -> bool {
		matches!(self.current(), Ctx::SLASH{..})
	}
}


#[cfg(test)]
mod test
{
	use super::*;
	
	#[test] fn force_pop_easy()
	{
		let mut ctx = ContextStack::new();

		ctx.push(Ctx::LEAVE { key: None });

		ctx.push(Ctx::SLASH { key: None });
		ctx.push(Ctx::ONLY);
		ctx.force_pop(Ctx::SLASH { key: None });
		assert_eq!( *ctx.current(), Ctx::LEAVE { key: None } );
	}

	#[test] fn force_pop_medium()
	{
		let mut ctx = ContextStack::new();
		ctx.push(Ctx::LEAVE { key: None });

		ctx.push(Ctx::SLASH { key: None });
		ctx.push(Ctx::ONLY);
		ctx.push(Ctx::SLASH { key: None });
		ctx.force_pop(Ctx::SLASH { key: None });
		assert_eq!( *ctx.current(), Ctx::ONLY );
		
		ctx.force_pop(Ctx::SLASH { key: None });
		assert_eq!( *ctx.current(), Ctx::LEAVE { key: None } );
	}

	#[test] fn force_pop_hard()
	{
		let mut ctx = ContextStack::new();
		ctx.push(Ctx::LEAVE { key: None });

		ctx.push(Ctx::SLASH { key: None });
		ctx.push(Ctx::ONLY);
		ctx.push(Ctx::ONLY);

		ctx.force_pop(Ctx::LEAVE { key: None });
		assert_eq!( *ctx.current(), Ctx::MARKDOWN );
	}
}
