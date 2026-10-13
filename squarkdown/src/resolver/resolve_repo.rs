use crate::prelude::*;
use crate::macros::*;

use git2 as git;

use std::path::{ Path };


/// Find the closest Git repository to `from`.
pub fn resolve_project_repo(from: impl AsRef<Path>) -> SquarkResult<git::Repository>
{
	let mut checked = vec![];

	for dir in from.as_ref().ancestors() {
		if let Ok(repo) = git::Repository::open(dir) {
			return Ok(repo);
		}
		checked.push(dir);
	}

	Err(SquarkError::Recoverable {
		msg: fmt!("could not find your project Git repository"),
		hint: str!(),
		debug: checked.into_iter().map(|d| slash!("checked: {}", d)).collect(),
	})
}
