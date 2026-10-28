use crate::utils::macros::*;


/// Remove whitespace from the end of `string`, in-place.
#[must_use]
pub fn trim_end(mut string: String) -> String
{
	string.truncate(string.trim_end().len());
	string
}

/// Try to slice `string` from the *end* so that it has at most `max_chars` Unicode characters (not graphemes).
/// 
/// Returns `None` if no truncation needs to happen.
pub fn truncated_end(string: &str, max_chars: usize) -> Option<&str>
{
	let (end, _) = string.char_indices().nth(max_chars)?;
	Some(&string[..end])
}

/// Try to slice `string` from the *start* so that it has at most `max_chars` Unicode characters (not graphemes).
/// 
/// Returns `None` if no truncation needs to happen.
pub fn truncated_start(string: &str, max_chars: usize) -> Option<&str>
{
	let (start, _) = string.char_indices().rev().nth(zero!(max_chars - 1))?;
	Some(&string[start..])
}
