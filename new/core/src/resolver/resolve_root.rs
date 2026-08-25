use crate::*;
use crate::utils::colours::*;

use std::path::PathBuf;


/// Find the root directory of the user's project to squarkup.
/// 
/// This is defined to be the closest directory that contains a `.squarkdown/` folder.
pub fn resolve_project_root() -> SquarkResult<PathBuf>
{
	let mut checked = vec![];

	let cwd = std::env::current_dir().map_err(err!())?;

	for dir in cwd.ancestors() {
		if all!(dir.join(".squarkdown") => .exists(), .is_dir()) {
			return Ok(dir.to_owned());
		}
		if all!(dir.join("squarkup.toml") => .exists(), .is_file()) {
			return Ok(dir.to_owned());
		}
		if all!(dir.join("squarkup.json") => .exists(), .is_file()) {
			return Ok(dir.to_owned());
		}

		checked.push(dir);
	}

	Err(SquarkError::Unrecoverable {
		msg: str!("could not find the root of your project"),
		hint: format!("make sure you have either a {W}.squarkdown/{G} folder, or a {W}squarkup.toml{G} or {W}squarkup.json{G} file, in the root of your project"),
		debug: checked.into_iter().map(|d| format!("checked {}", d.display())).collect(),
	})
}
