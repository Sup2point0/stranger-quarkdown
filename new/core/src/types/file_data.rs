use crate::parser::FieldValues;

use std::collections::HashMap;


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
		fields: HashMap<String, FieldValues>,
	) -> Self
	{
		unimplemented!()
	}
}
