use crate::core::*;
use crate::macros::*;

use lazy_static::lazy_static;

use std::path::PathBuf;



lazy_static!
{
	pub static ref TESTS: PathBuf = std::env::current_dir().unwrap().join("tests/test-project");

	pub static ref TEST_FILE: PathBuf = dir!(TESTS / "unit.md");
	
	/// A barebones testing squarkup config used for unit tests.
	pub static ref TEST_CONFIG: SquarkupConfig = SquarkupConfig::init_defaults(&TESTS, &TESTS);

	pub static ref TEST_PAGE: PageData = PageData::default();

	pub static ref TEST_SITE: SiteData = SiteData::new();
}
