use crate::types::Shard;

use std::collections::{ HashSet };
use std::path::{ PathBuf };


/// Metadata for a tag, stored in [`SiteData`](crate::types::SiteData).
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct TagData
{
	/// The `filepath` of the page that acts as an index page for this tag.
	pub index: PathBuf,

	/// `filepath`s of the pages 
	pub pages: HashSet<Shard>,
}
