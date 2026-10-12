use crate::types::ContextStack;

use enum_display::EnumDisplay;


#[derive(Clone, Default, PartialEq, Eq, Debug, EnumDisplay)]
pub enum RenderCtx
{
	#[default]
	#[display("")]
	MARKDOWN,

	#[display("comment")]
	COMMENT,

	#[display("code block")]
	CODE,

	// TODO stringify with key

	/* NOTE: LEAVE has higher priority than SLASH. A SLASH should always be ignored if inside a LEAVE context. */

	#[display("#SQUARK leave")]
	LEAVE { key: Option<String> },

	#[display("#SQUARK slash")]
	SLASH { key: Option<String> },

	#[display("#SQUARK only")]
	ONLY,
}


impl ContextStack<RenderCtx>
{
	/// Is [`RenderCtx::CODE`] in the context stack (at any depth)?
	pub fn is_code(&self) -> bool {
		self.stack().iter().any(|ctx| matches!(ctx, RenderCtx::CODE{..}))
	}

	/// Is [`RenderCtx::LEAVE`] in the context stack (at any depth)?
	pub fn has_leave(&self) -> bool {
		self.stack().iter().any(|ctx| matches!(ctx, RenderCtx::LEAVE{..}))
	}

	/// Is [`RenderCtx::SLASH`] in the context stack (at any depth)?
	pub fn has_slash(&self) -> bool {
		self.stack().iter().any(|ctx| matches!(ctx, RenderCtx::SLASH{..}))
	}

	pub fn printed(&self) -> Vec<String> {
		self.printed_with_prefix("in")
	}
}
