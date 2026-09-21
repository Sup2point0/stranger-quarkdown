/// A first-in last-out stack for tracking contexts, with an always-active base context.
#[derive(Clone, Default, Debug)]
pub struct ContextStack<Ctx>
{
	base: Ctx,
	stack: Vec<Ctx>,
}

impl<Ctx> ContextStack<Ctx>
	where Ctx: Default
{
	pub fn new() -> Self {
		Self::default()
	}
}

impl<Ctx> ContextStack<Ctx>
{
	pub fn stack(&self) -> &[Ctx] {
		&self.stack
	}

	/// What's the current context?
	pub fn current(&self) -> &Ctx {
		self.stack.last().unwrap_or(&self.base)
	}

	pub fn push(&mut self, ctx: Ctx) {
		self.stack.push(ctx);
	}
}

impl<Ctx> ContextStack<Ctx>
	where Ctx: Default + PartialEq + Eq
{
	// TODO use result when forced
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
	pub fn force_pop(&mut self, ctx: Ctx) -> bool
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


// == TESTS == //

#[cfg(test)] use crate::renderer::RenderCtx;


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
	assert_eq!( *ctx.current(), RenderCtx::MARKDOWN );
}
