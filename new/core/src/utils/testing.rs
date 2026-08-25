use lazy_static::lazy_static;

use crate::{
	config::*,
};

use std::path::PathBuf;



lazy_static!
{
	pub static ref TESTS: PathBuf = std::env::current_dir().unwrap().join("tests/test-project");
	
	/// A barebones testing squarkup config used for unit tests.
	pub static ref TEST_CONFIG: SquarkupConfig = SquarkupConfig::init_defaults(&TESTS, &TESTS);
}
