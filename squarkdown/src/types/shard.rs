/// A unique stable identifier for a page, normalised from its filepath.
#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Serialize)]
pub struct Shard(String);

impl<T> From<T> for Shard
	where T: Into<String>
{
	fn from(value: T) -> Self {
		Self(value.into())
	}
}

impl std::borrow::Borrow<str> for Shard
{
	fn borrow(&self) -> &str {
		&self.0
	}
}

impl std::fmt::Display for Shard
{
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
	{
		write!(f, "{}", self.0)
	}
}
