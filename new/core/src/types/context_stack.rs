use crate::prelude::*;
use crate::macros::*;


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
	where Ctx: PartialEq + Eq + std::fmt::Debug
{
	/// Pop `ctx` from the stack, as deep as possible.
	/// 
	/// If there are multiple consecutive occurrences of `ctx`, this pops all of them. For instance, popping `SLASH` from `[LEAVE, SLASH, SLASH]` results in `[LEAVE]`.
	/// 
	/// Errors if `ctx` is not the current context.
	pub fn try_pop(&mut self, ctx: Ctx) -> SquarkResult
	{
		if *self.current() != ctx {
			return Err(SquarkError::Recoverable {
				msg: fmt!("failed to pop context: {ctx:?}"),
				hint: str!("contexts must always be balanced"),
				debug: vec![],
			});
		}

		while *self.current() == ctx {
			self.stack.pop();
		}

		Ok(())
	}
}

impl<Ctx> ContextStack<Ctx>
	where Ctx: std::fmt::Display
{
	/// Print the stack, top-down from the last pushed context.
	pub fn printed(&self) -> Vec<String>
	{
		self.prints().collect()
	}

	/// Print the stack, top-down from the last pushed context.
	pub fn prints(&self) -> impl Iterator<Item = String>
	{
		self.stack().iter().rev().map(ToString::to_string)
	}
}


// == TESTS == //

#[cfg(test)] use crate::renderer::RenderCtx;

#[cfg(test)] use assertables::*;


#[cfg(test)]
mod try_pop {
	use super::*;

	#[test] fn basic()
	{
		let mut ctx = ContextStack::new();
		ctx.push(RenderCtx::LEAVE { key: None });
		ctx.push(RenderCtx::SLASH { key: None });

		let r = ctx.try_pop(RenderCtx::SLASH { key: None });
		assert_ok!( &r );
		assert_eq!( *ctx.current(), RenderCtx::LEAVE { key: None } );
	}

	#[test] fn multiple()
	{
		let mut ctx = ContextStack::new();
		ctx.push(RenderCtx::LEAVE { key: None });
		ctx.push(RenderCtx::SLASH { key: None });
		ctx.push(RenderCtx::SLASH { key: None });

		let r = ctx.try_pop(RenderCtx::SLASH { key: None });
		assert_ok!( &r );
		assert_eq!( *ctx.current(), RenderCtx::LEAVE { key: None } );
	}

	#[test] fn stops()
	{
		let mut ctx = ContextStack::new();
		ctx.push(RenderCtx::LEAVE { key: None });
		ctx.push(RenderCtx::SLASH { key: None });
		ctx.push(RenderCtx::LEAVE { key: None });

		let r = ctx.try_pop(RenderCtx::LEAVE { key: None });
		assert_ok!( &r );
		assert_eq!( *ctx.current(), RenderCtx::SLASH { key: None } );
	}

	#[test] fn fails()
	{
		let mut ctx = ContextStack::new();
		ctx.push(RenderCtx::LEAVE { key: None });
		ctx.push(RenderCtx::SLASH { key: None });

		let r = ctx.try_pop(RenderCtx::LEAVE { key: None });
		assert_err!( &r );
		assert_eq!( *ctx.current(), RenderCtx::SLASH { key: None } );
	}
}
