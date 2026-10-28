//! This module provides utility constants for unit tests.

use crate::prelude::*;

use lazy_static::lazy_static;
use path_macro::path;

use std::path::PathBuf;



lazy_static!
{
	pub static ref TESTS:        PathBuf = path!(std::env::current_dir().unwrap() / "tests");
	pub static ref TEST_PROJECT: PathBuf = path!(*TESTS / "test-project");
	pub static ref TEST_FILE:    PathBuf = path!(*TEST_PROJECT / "unit.md");
	
	/// A barebones testing squarkup config for unit tests.
	pub static ref TEST_CONFIG: SquarkupConfig = SquarkupConfig::init_defaults(&TEST_PROJECT, &TEST_PROJECT);

	/// Dummy page data for unit tests (with invalid paths).
	pub static ref TEST_PAGE: PageData = PageData::default();

	pub static ref TEST_SITE_DATA: SiteData = SiteData::new();
}
