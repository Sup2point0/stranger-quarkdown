use squarkdown::utils::macros::*;

use lazy_static::lazy_static;
use path_macro::path;

use std::fs;
use std::io;
use std::path::{ Path, PathBuf };
use std::process;


lazy_static!
{
	pub static ref BENCHES: PathBuf = std::env::current_dir().unwrap();
}


const BIN: &str = env!("CARGO_BIN_EXE_squarkdown");

fn main() -> io::Result<()>
{
	create_source_dirs()?;
	
	let r = process::Command::new(BIN)
		.current_dir(BENCHES.clone())
		.output()
	;

	let process::Output{ stdout, stderr, .. } = r?;

	let out = String::from_utf8(stdout).unwrap();
	print!("{out}");

	if !stderr.is_empty() {
		let err = String::from_utf8(stderr).unwrap();
		print!("{err}");
	}

	Ok(())
}


const SOURCE_DIR_FILES: usize = 1000;

const SOURCE_DIRS_DEPTH: usize = 3;

const SOURCE_DIR_NAMES: [&str; 6] = [
	"prot",
	"deut",
	"trit",
	"quat",
	"quint",
	"hexa",
];

fn create_source_dirs() -> io::Result<()>
{
	create_source_dir_from(&BENCHES, 0)
}

fn create_source_dir_from(folder: &Path, depth: usize) -> io::Result<()>
{
	if depth >= SOURCE_DIRS_DEPTH { return Ok(()); }

	for dir in SOURCE_DIR_NAMES {
		let folder = path!(folder / dir);
		assert!(folder.starts_with(path!(*BENCHES / "bench-sources")));

		// fs::remove_dir_all(&folder)?;
		fs::create_dir(&folder)?;

		for i in 0..SOURCE_DIR_FILES {
			fs::write(
				path!(folder / fmt!("page-{i}.md")),
				fmt!("
					# Page #{i}
					<!-- #SQUARK live! -->
				")
			)?;
		}

		create_source_dir_from(&folder, depth + 1)?;
	}

	Ok(())
}
