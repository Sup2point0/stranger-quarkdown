use crate::utils::macros::*;


#[derive(Clone, Copy, Debug, PartialEq, Eq, enum_stringify::EnumStringify)]
#[enum_stringify(case = "flat")]
pub enum CleanseOperation
{
	/// Replace non-tag `<>` with `&lt;`, `&gt;` for HTML safety.
	#[enum_stringify(case = "flat")]
	ANGLES,

	/// Replace `{}` with `&lbrace;`, `&rbrace;` for HTML safety.
	BRACES,

	/// Strip comments.
	COMMENTS,

	/// Remove `\n<br>\n` large line breaks.
	LINE_BREAKS,
}
