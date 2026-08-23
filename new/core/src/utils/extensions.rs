/// Remove whitespace from the end of `string`, in-place.
pub fn trim_end(mut string: String) -> String
{
	string.truncate(string.trim_end().len());
	string
}
