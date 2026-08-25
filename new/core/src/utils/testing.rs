use lazy_static::lazy_static;

use super::macros::*;
use crate::{
	config::*,
};

use std::path::PathBuf;



lazy_static!
{
	pub static ref TESTS: PathBuf = std::env::current_dir().unwrap().join("tests/test-project");
	
	/// A testing squarkup config used for unit tests.
	pub static ref TEST_CONFIG: SquarkupConfig = SquarkupConfig
	{
		paths: PathsConfig {
			root: TESTS.clone(),
			site: dir!(TESTS / "test-site"),
			dest: dir!(TESTS / "test-site/src/routes/test-gen"),
			sources: vec![TESTS.clone()],
			include: vec![str!(r#"\.txt$"#)],
			include_patterns: vec![],
			exclude: vec![str!("ignored"), str!(r#"/_*\.md"#)],
			exclude_patterns: vec![],
			default_exclude: true,
		},
		errors: ErrorConfig {
			on_error: ErrorAction::KILL,
			on_file_exists: FileAction::OVERWRITE,
		}
	};
}
