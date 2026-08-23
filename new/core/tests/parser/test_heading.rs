use std::io::Cursor;

use squarkup::parser::CharmParser;
use squarkup::utils::*;


#[test] fn test_basic()
{
	let source = Cursor::new("# Sup\n".as_bytes());
	let parser = CharmParser::init(source, &TEST_CONFIG);
}
