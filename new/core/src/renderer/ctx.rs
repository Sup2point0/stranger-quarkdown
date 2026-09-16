#[allow(clippy::upper_case_acronyms)]
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub enum RenderCtx
{
	#[default]
	MARKDOWN,

	COMMENT,
	CODE,
	LEAVE { key: Option<String> },
	SLASH { key: Option<String> },
	ONLY,
}
