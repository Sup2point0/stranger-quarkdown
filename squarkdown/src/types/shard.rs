use std::borrow::Borrow;


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

impl Borrow<str> for Shard
{
	fn borrow(&self) -> &str {
		&self.0
	}
}
