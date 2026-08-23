use std::collections::HashMap;

use super::SquarkValue;


pub struct FileData
{
	pub dest: String,
	pub heading: Option<String>,
	// TODO
}

impl FileData
{
	pub fn init(flags: Vec<String>, fields: HashMap<String, SquarkValue>) -> Self
	{
		unimplemented!()
	}
}
