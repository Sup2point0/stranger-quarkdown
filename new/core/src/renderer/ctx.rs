use crate::types::ContextStack;


#[allow(clippy::upper_case_acronyms)]
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub enum RenderCtx
{
	#[default]
	MARKDOWN,

	HEADING,
	COMMENT,
	CODE,
	LEAVE { key: Option<String> },
	SLASH { key: Option<String> },
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
