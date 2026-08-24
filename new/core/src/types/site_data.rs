use crate::FileData;


pub struct SiteData
{
	pub files: Vec<FileData>,
}

impl SiteData
{
	pub fn new() -> Self
	{
		Self {
			files: vec![],
		}
	}
}
