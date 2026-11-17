use crate::types::Shard;

use std::collections::{ HashSet };


/// Metadata for a tag, stored in [`SiteData`](crate::types::SiteData).
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct TagData
{
	///The page that acts as an index page for this tag.
	pub index: Shard,

	/// The pages with this tag.
	pub pages: HashSet<Shard>,
}
