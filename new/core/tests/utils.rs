use squarkdown::macros::*;

use lazy_static::lazy_static;

use std::fs::{ File };
use std::io::{ Read };
use std::path::{ PathBuf };
use std::assert_matches;


lazy_static! {
	pub static ref TESTS: PathBuf
		= std::env::current_dir().unwrap().join("tests");
	
	pub static ref TEST_SITE: PathBuf
		= TESTS.join("test-project/src/routes");
}


const BIN: &'static str = env!("CARGO_BIN_EXE_squarkdown");

/// Run Squarkdown from `path`, relative to `tests/`.
pub fn squarkup_from(path: &str) -> std::process::ExitStatus
{
	let r = std::process::Command::new(BIN)
		.current_dir(TESTS.join(path))
		.status();

	assert_matches!( r, Ok(..) );
	r.unwrap()
}


/// Read the content of the file at `path`, relative to `tests/test-project/src/routes/`.
pub fn read_file(path: &str) -> String
{
	let path = TEST_SITE.join(path);
	if !path.exists() {
		panic!("{}", slash!("no file found at: {}", path));
	}

	let mut file = File::open(path).unwrap();

	let mut out = str!();
	file.read_to_string(&mut out).unwrap();

	out
}
