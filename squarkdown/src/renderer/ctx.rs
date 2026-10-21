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

	#[display("link")]
	LINK,

	/// A `[Markdown](markdown)` link is being rewritten to `<a href="HTML">HTML</a>` format.
	#[display("link to rewrite")]
	LINK_REWRITE,

	#[display("image")]
	IMAGE_LINK,

	// TODO stringify with key

	/* NOTE: LEAVE has higher priority than SLASH. A SLASH should always be ignored if inside a LEAVE context. */

	#[display("#SQUARK leave")]
	LEAVE {
		key: Option<String>,
	},

	#[display("#SQUARK slash")]
	SLASH {
		key: Option<String>,
	},

	#[display("#SQUARK only")]
	ONLY,
}


impl ContextStack<RenderCtx>
{
	/// Is [`RenderCtx::CODE`] in the context stack (at any depth)?
	pub fn is_code(&self) -> bool {
		self.stack().iter().any(|ctx| matches!(ctx, RenderCtx::CODE))
	}
	
	/// Is [`RenderCtx::LINK`], [`RenderCtx::LINK_REWRITE`] or [`RenderCtx::IMAGE_LINK`] in the context stack (at any depth)?
	pub fn is_link(&self) -> bool {
		self.stack().iter().any(|ctx| matches!(ctx,
			  RenderCtx::LINK
			| RenderCtx::LINK_REWRITE
			| RenderCtx::IMAGE_LINK
		))
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
		self.printed_with_prefix("in ")
	}
}
