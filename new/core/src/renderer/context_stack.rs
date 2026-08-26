#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ctx {
	MARKDOWN,
	CODE_INLINE,
	CODE_BLOCK,
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

	pub fn force_pop(&mut self, ctx: Ctx)
	{
		if let Some(idx) = self.stack.iter().rev().position(|c| *c == ctx) {
			self.stack.truncate(idx - 1);
		}
	}
}
