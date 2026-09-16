#[derive(Debug, Clone)]
pub struct ContextStack<T>
{
	base: T,
	stack: Vec<T>,
}

/// Constructors
impl<T> ContextStack<T>
	where T: Default + PartialEq + Eq
{
	pub fn new() -> Self {
		Self {
			base: T::default(),
			stack: vec![],
		}
	}
}

/// Implementation
impl<T> ContextStack<T>
	where T: Default + PartialEq + Eq
{
	pub fn stack(&self) -> &[T] {
		&self.stack
	}

	/// What's the current context?
	pub fn current(&self) -> &T {
		self.stack.last().unwrap_or(&self.base)
	}

	pub fn push(&mut self, ctx: T) {
		self.stack.push(ctx);
	}

	/// Pop `ctx` from the stack if it is the currently active context, taking keys into account.
	pub fn try_pop(&mut self, ctx: T) -> bool
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
	pub fn force_pop(&mut self, ctx: T) -> bool
	{
		if let Some(idx) = self.stack.iter().rposition(|c| *c == ctx) {
			self.stack.truncate(idx);
		} else {
			return false;
		}

		while *self.current() == ctx {
			self.stack.pop();
		}

		true
	}
}

impl ContextStack<RenderCtx>
{
	/// Is the current context `RenderCtx::LEAVE`?
	pub fn is_leave(&self) -> bool {
		matches!(self.current(), RenderCtx::LEAVE{..})
	}

	/// Is the current context `RenderCtx::SLASH`?
	pub fn is_slash(&self) -> bool {
		matches!(self.current(), RenderCtx::SLASH{..})
	}
}


#[cfg(test)]
mod test
{
	use super::*;
	use crate::renderer::RenderCtx;
	
	#[test] fn force_pop_easy()
	{
		let mut ctx = ContextStack::new();

		ctx.push(RenderCtx::LEAVE { key: None });

		ctx.push(RenderCtx::SLASH { key: None });
		ctx.push(RenderCtx::ONLY);
		ctx.force_pop(RenderCtx::SLASH { key: None });
		assert_eq!( *ctx.current(), RenderCtx::LEAVE { key: None } );
	}

	#[test] fn force_pop_medium()
	{
		let mut ctx = ContextStack::new();
		ctx.push(RenderCtx::LEAVE { key: None });

		ctx.push(RenderCtx::SLASH { key: None });
		ctx.push(RenderCtx::ONLY);
		ctx.push(RenderCtx::SLASH { key: None });
		ctx.force_pop(RenderCtx::SLASH { key: None });
		assert_eq!( *ctx.current(), RenderCtx::ONLY );
		
		ctx.force_pop(RenderCtx::SLASH { key: None });
		assert_eq!( *ctx.current(), RenderCtx::LEAVE { key: None } );
	}

	#[test] fn force_pop_hard()
	{
		let mut ctx = ContextStack::new();
		ctx.push(RenderCtx::LEAVE { key: None });

		ctx.push(RenderCtx::SLASH { key: None });
		ctx.push(RenderCtx::ONLY);
		ctx.push(RenderCtx::ONLY);

		ctx.force_pop(RenderCtx::LEAVE { key: None });
		assert_eq!( *ctx.current(), T::MARKDOWN );
	}
}
