use super::FileError;
use crate::utils::macros::*;


#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CleanseOperation
{
	/// Replace non-tag `<>` with `&lt;`, `&gt;` for HTML safety.
	ANGLES,

	/// Replace `{}` with `&lbrace;`, `&rbrace;` for HTML safety.
	BRACES,

	/// Strip comments.
	COMMENTS,

	/// Remove `\n<br>\n` large line breaks.
	#[allow(non_camel_case_types)]
	LINE_BREAKS,
}

impl TryFrom<String> for CleanseOperation
{
	type Error = FileError;

	fn try_from(mut value: String) -> Result<Self, Self::Error>
	{
		value.make_ascii_lowercase();

		match value.as_str()
		{
			"angles" => Ok(Self::ANGLES),
			"braces" => Ok(Self::BRACES),
			"comments" => Ok(Self::COMMENTS),
			"line-breaks" | "line breaks" => Ok(Self::LINE_BREAKS),
			
			_ => Err(FileError::InvalidValue { field: str!("cleanse"), value })
		}
	}
}
