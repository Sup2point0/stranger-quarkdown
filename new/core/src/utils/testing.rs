use std::path::PathBuf;

use lazy_static::lazy_static;

use super::macros::*;

use crate::types::*;


lazy_static!
{
	pub static ref TESTS: PathBuf = std::env::current_dir().unwrap();
	
	/// A testing squarkup config used for unit tests.
	pub static ref TEST_CONFIG: SquarkupConfig = SquarkupConfig
	{
		paths: PathsConfig {
			repo: dir!(TESTS / "test-project"),
			site: dir!(TESTS / "test-project/test-site"),
			dest: dir!(TESTS / "test-project/test-site/src/routes/test-gen"),
			sources: vec![str!("content")],
			exclude: vec![str!("ignored")],
		}
	};
}
