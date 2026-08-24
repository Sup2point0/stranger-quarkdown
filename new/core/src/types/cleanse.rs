use crate::utils::macros::*;

use enum_stringify::EnumStringify;


#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[derive(EnumStringify)] #[enum_stringify(case = "flat")]
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
	#[allow(non_camel_case_types)]
	LINE_BREAKS,
}
