use crate::types::Shard;

use std::collections::{ HashSet };


/**
 * A series of related pages, grouped in [`SiteData.series`](crate::types::SiteData).
 */
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Series
{
	///The page that acts as an index (‘home’) page for this series.
	pub index_page: Shard,

	/// The pages that are part of this series.
	pub pages: HashSet<Shard>,
}
