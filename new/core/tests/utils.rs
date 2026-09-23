use squarkdown::macros::*;

use assertables::*;
use lazy_static::lazy_static;
use path_macro::path;

use std::fs::{ self, File };
use std::io::{ self, Read };
use std::path::{ PathBuf };
use std::process;


#[macro_export]
macro_rules! assert_not {
	($($tokens:tt)*) => {
		assert!( !($($tokens)*) )
	};
}



lazy_static! {
	pub static ref TESTS: PathBuf
		= path!(std::env::current_dir().unwrap() / "tests");
	
	pub static ref TEST_SITE: PathBuf
		= path!(*TESTS / "test-project");
	
	pub static ref TEST_ROUTES: PathBuf
		= path!(*TEST_SITE / "src/routes");
}


const BIN: &'static str = env!("CARGO_BIN_EXE_squarkdown");

/// Run Squarkdown from `path`, relative to `tests/`.
pub fn squarkup_from(path: &str) -> process::ExitStatus
{
	let r = process::Command::new(BIN)
		.current_dir(path!(*TESTS / path))
		.status();

	assert_ok!( &r );
	r.unwrap()
}

#[macro_export]
macro_rules! squarkup {
	($path:literal $(, $arg:literal)*) => {
		{
			let bin = env!("CARGO_BIN_EXE_squarkdown");
			let r = std::process::Command::new(bin)
				.current_dir(TESTS.join($path))
				$( .arg($arg) )*
				.status();

			assertables::assert_ok!( &r );
			r.unwrap()
		}
	};
}

/// Run Squarkdown from `path`, relative to `tests/`, capturing what it prints to stdout for querying.
pub fn capture_squarkup_from(path: &str) -> (process::ExitStatus, String)
{
	let r = process::Command::new(BIN)
		.current_dir(path!(*TESTS / path))
		.output();

	assert_ok!( &r );
	let process::Output{ status, stdout, .. } = r.unwrap();

	let out = str::from_utf8(&stdout).unwrap().to_string();
	print!("{out}");

	(status, out)
}


/// Recursively delete all files under `path`, except `.gitkeep`.
pub fn clear_files(path: &str) -> io::Result<()>
{
	let path = path!(*TEST_ROUTES / path);
	if !path.exists() {
		panic!("{}", slash!("no folder found at: {}", path));
	}
	if !path.is_dir() {
		panic!("{}", slash!("{} is not a folder", path))
	}
	
	if !path!(path / ".gitkeep").exists() {
		panic!("{}", slash!("danger: {} does not contain a .gitkeep file", path))
	}

	fs::remove_dir_all(&path)?;
	fs::create_dir(&path)?;
	File::create(path!(path / ".gitkeep"))?;

	Ok(())
}


/// Read the content of the file at `path`, relative to `tests/test-project/src/routes/`.
pub fn read_file(path: &str) -> String
{
	let path = path!(*TEST_ROUTES / path);
	if !path.exists() {
		panic!("{}", slash!("no file found at: {}", path));
	}

	fs::read_to_string(path).unwrap()
}
