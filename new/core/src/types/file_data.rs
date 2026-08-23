use std::collections::HashMap;

use tinyvec::TinyVec;


pub struct FileData
{
	pub dest: String,
	pub heading: Option<String>,
	// TODO
}

impl FileData
{
	pub fn init(
		flags: Vec<String>,
		fields: HashMap<String, TinyVec<[String; 1]>>,
	) -> Self
	{
		unimplemented!()
	}
}
