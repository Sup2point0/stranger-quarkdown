use crate::core::*;

use lazy_static::lazy_static;
use path_macro::path;

use std::path::PathBuf;



lazy_static!
{
	pub static ref TESTS: PathBuf = path!(std::env::current_dir().unwrap() / "tests/test-project");

	pub static ref TEST_FILE: PathBuf = path!(*TESTS / "unit.md");
	
	/// A barebones testing squarkup config used for unit tests.
	pub static ref TEST_CONFIG: SquarkupConfig = SquarkupConfig::init_defaults(&TESTS, &TESTS);

	pub static ref TEST_PAGE: PageData = PageData::default();

	pub static ref TEST_SITE: SiteData = SiteData::new();
}


#[macro_export]
macro_rules! assert_not {
	($($tokens:tt)*) => {
		assert!( !($($tokens)*) )
	};
} pub use assert_not;
