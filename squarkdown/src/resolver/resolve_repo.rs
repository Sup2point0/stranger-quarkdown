use crate::prelude::*;
use crate::utils::macros::*;

use git2 as git;
use path_clean::PathClean;

use std::path::{ Path };


/// Find the closest Git repository to `from`, stopping at `root`.
pub fn resolve_project_repo(
	from: impl AsRef<Path>,
	root: impl AsRef<Path>
) -> SquarkResult<git::Repository>
{
	let root = root.as_ref().clean();

	let mut checked = vec![];

	for dir in from.as_ref().ancestors() {
		if let Ok(repo) = git::Repository::open(dir) {
			return Ok(repo)
		}
		if dir == root {
			break;
		}
		checked.push(dir);
	}

	Err(SquarkError::Recoverable {
		msg: fmt!("could not find your project Git repository"),
		hint: str!(),
		debug: checked.into_iter().map(|d| slash!("checked: {}", d)).collect(),
	})
}
