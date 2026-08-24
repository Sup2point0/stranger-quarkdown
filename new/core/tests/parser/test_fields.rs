use std::io::Cursor;

use squarkup::parser::CharmParser;
use squarkup::utils::testing::*;

use crate::macros::*;
// ## Heading

#[test] fn test_basic()
{
	let source = Cursor::new("
# Test
<!-- #SQUARK live!
| dest = test
-->
	".trim());

	let mut parser = CharmParser::init(source, &TEST_CONFIG).unwrap();
	let file_data = parser.parse().unwrap();

	assert_eq!( file_data.heading, Some(str!("Test")) );
	assert_eq!( file_data.dest, str!("test") );
}
