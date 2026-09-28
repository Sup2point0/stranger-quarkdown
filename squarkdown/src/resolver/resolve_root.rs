use crate::prelude::*;
use crate::colours::*;
use crate::macros::*;

use path_macro::path;

use std::path::PathBuf;


/// Find the root directory of the user's project to squarkup.
/// 
/// This is defined to be the closest directory that contains a `.squarkdown/` folder.
pub fn resolve_project_root() -> SquarkResult<PathBuf>
{
	let mut checked = vec![];

	let cwd = std::env::current_dir()?;

	for dir in cwd.ancestors() {
		if path!(dir / ".squarkdown").is_dir() {
			return Ok(dir.to_owned());
		}
		if path!(dir / "squarkup.toml").is_file() {
			return Ok(dir.to_owned());
		}
		if path!(dir / "squarkup.json").is_file() {
			return Ok(dir.to_owned());
		}

		checked.push(dir);
	}

	Err(SquarkError::Unrecoverable {
		msg: str!("could not find the root of your project"),
		hint: fmt!("make sure you have either a {W}.squarkdown/{G} folder, or a {W}squarkup.toml{G} or {W}squarkup.json{G} file, in the root of your project"),
		debug: checked.into_iter().map(|d| fmt!("checked {}", d.display())).collect(),
	})
}
