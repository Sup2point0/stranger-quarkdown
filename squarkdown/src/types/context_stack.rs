use crate::prelude::*;
use crate::utils::macros::*;

use eq_discriminant::eq_discr;


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

	pub fn is_empty(&self) -> bool {
		self.stack.is_empty()
	}

	pub fn push(&mut self, ctx: Ctx) {
		self.stack.push(ctx);
	}
}

impl<Ctx> ContextStack<Ctx>
	where Ctx: PartialEq + std::fmt::Debug
{
	pub fn contains(&self, ctx: &Ctx) -> bool
	 {
		/* NOTE: Favour reverse search since we're more likely to be querying down the stack*/
		self.stack.iter().rev().any(|c| c == ctx)
	}

	/// Pop any `ctx` variant from the stack, as deep as possible.
	/// 
	/// This is the non-keyed version of [`try_pop_exact`](Self::try_pop_exact), only checking the context enum variant but disregarding any attached fields.
	/// 
	/// If there are multiple consecutive occurrences of `ctx`, this pops all of them. For instance, popping `SLASH` from `[LEAVE, SLASH, SLASH]` results in `[LEAVE]`.
	/// 
	/// Errors if `ctx` is not the current context variant.
	pub fn try_pop_any(&mut self, ctx: &Ctx) -> SquarkResult<Ctx>
	{
		let Some(mut out) = self.stack.pop_if(|c| eq_discriminant::eq_discriminant(c, ctx))
			else {
				return Err(SquarkError::Recoverable {
					msg: fmt!("failed to pop context: {ctx:?}"),
					hint: str!("contexts must always be balanced"),
					debug: vec![],
				});
			};

		while let Some(popped) = self.stack.pop_if(|c| eq_discr!(c, ctx)) {
			out = popped;
		}

		Ok(out)
	}

	/// Pop exactly `ctx` from the stack, as deep as possible.
	/// 
	/// This is the keyed version of [`try_pop_any`](Self::try_pop_any), requiring the fields (e.g. `key`) to be equal.
	/// 
	/// If there are multiple consecutive occurrences of `ctx`, this pops all of them. For instance, popping `SLASH` from `[LEAVE, SLASH, SLASH]` results in `[LEAVE]`.
	/// 
	/// Errors if `ctx` is not the exact current context.
	pub fn try_pop_exact(&mut self, ctx: &Ctx) -> SquarkResult<Ctx>
	{
		let Some(mut out) = self.stack.pop_if(|c| c == ctx)
			else {
				return Err(SquarkError::Recoverable {
					msg: fmt!("failed to pop context: {ctx:?}"),
					hint: str!("contexts must always be balanced"),
					debug: vec![],
				});
			};

		while let Some(popped) = self.stack.pop_if(|c| c == ctx) {
			out = popped;
		}

		Ok(out)
	}
}

impl<Ctx> ContextStack<Ctx>
	where Ctx: std::fmt::Display
{
	/// Print the stack, top-down from the last pushed context.
	pub fn printed_with_prefix(&self, prefix: &str) -> Vec<String>
	{
		self.prints_with_prefix(prefix).collect()
	}

	/// Prints the stack, top-down from the last pushed context.
	pub fn prints_with_prefix(&self, prefix: &str) -> impl DoubleEndedIterator<Item = String>
	{
		self.stack().iter().rev().map(move |ctx| fmt!("{prefix}{ctx}"))
	}
}

impl<Ctx> std::ops::Deref for ContextStack<Ctx>
{
	type Target = [Ctx];

	fn deref(&self) -> &Self::Target {
		&self.stack
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

		let r = ctx.try_pop_exact(&RenderCtx::SLASH { key: None });
		assert_ok!( &r );
		assert_eq!( *ctx.current(), RenderCtx::LEAVE { key: None } );
	}

	#[test] fn multiple()
	{
		let mut ctx = ContextStack::new();
		ctx.push(RenderCtx::LEAVE { key: None });
		ctx.push(RenderCtx::SLASH { key: None });
		ctx.push(RenderCtx::SLASH { key: None });

		let r = ctx.try_pop_exact(&RenderCtx::SLASH { key: None });
		assert_ok!( &r );
		assert_eq!( *ctx.current(), RenderCtx::LEAVE { key: None } );
	}

	#[test] fn stops()
	{
		let mut ctx = ContextStack::new();
		ctx.push(RenderCtx::LEAVE { key: None });
		ctx.push(RenderCtx::SLASH { key: None });
		ctx.push(RenderCtx::LEAVE { key: None });

		let r = ctx.try_pop_exact(&RenderCtx::LEAVE { key: None });
		assert_ok!( &r );
		assert_eq!( *ctx.current(), RenderCtx::SLASH { key: None } );
	}

	#[test] fn fails()
	{
		let mut ctx = ContextStack::new();
		ctx.push(RenderCtx::LEAVE { key: None });
		ctx.push(RenderCtx::SLASH { key: None });

		let r = ctx.try_pop_exact(&RenderCtx::LEAVE { key: None });
		assert_err!( &r );
		assert_eq!( *ctx.current(), RenderCtx::SLASH { key: None } );
	}
}
