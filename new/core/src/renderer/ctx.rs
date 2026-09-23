use crate::types::ContextStack;

use enum_display::EnumDisplay;


#[allow(clippy::upper_case_acronyms)]
#[derive(Clone, Default, PartialEq, Eq, Debug, EnumDisplay)]
pub enum RenderCtx
{
	#[default]
	#[display("")]
	MARKDOWN,

	#[display("in comment")]
	COMMENT,

	#[display("in code block")]
	CODE,

	// TODO stringify with key

	#[display("in #SQUARK leave")]
	LEAVE { key: Option<String> },

	#[display("in #SQUARK slash")]
	SLASH { key: Option<String> },

	#[display("in #SQUARK only")]
	ONLY,
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
