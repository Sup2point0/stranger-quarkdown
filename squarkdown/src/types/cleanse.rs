#[derive(Clone, Copy, Debug, PartialEq, Eq, enum_stringify::EnumStringify)]
#[enum_stringify(case = "kebab")]
pub enum CleanseOperation
{
	/// Replace non-tag `<>` with `&lt;`, `&gt;` for HTML safety.
	ANGLES,

	/// Replace `{}` with `&lbrace;`, `&rbrace;` for HTML safety.
	BRACES,

	/// Remove `\n<br>\n` large line breaks.
	LINE_BREAKS,
}
