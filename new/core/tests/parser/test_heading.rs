use std::fs::File;

use squarkup::parser::CharmParser;

use crate::shared::*;


#[test] fn test_basic()
{
	let source = Cursor::new("# Sup\n".as_bytes());
	let parser = CharmParser::init(source, &TEST_CONFIG);
}
