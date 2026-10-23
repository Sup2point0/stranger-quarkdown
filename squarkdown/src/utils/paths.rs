use super::macros::slash;
use crate::prelude::*;

use map_or_original::MapOrOriginal;

use std::io;
use std::fs;
use std::path::Path;


/// Check if `dir` is an empty directory.
pub fn is_empty_dir(dir: impl AsRef<Path>) -> SquarkResult<bool>
{
	Ok(fs::read_dir(&dir)?.next().is_none())
}

/// Make `path` relative by stripping a leading path separator.
/// 
/// This is very important to ensure `base.join(path)` remains safe.
/// 
/// For instance:
/// - `"/base/path/".join("rel/path")` resolves to `/base/path/rel/path`, happy days.
/// - `"/base/path/".join("/rel/path")` resolves to `/rel/path`.
///   - With a leading separator, the latter is treated as an absolute path, overriding the base path.
#[must_use]
pub fn to_rel(path: &str) -> &str
{
	path
		.trim_start_matches(['/', '\\'])
		.trim_start_matches("./")
}

/// Display an `absolute` path relative to `base`.
#[must_use]
pub fn display_rel(
	absolute: impl AsRef<Path>,
	base: impl AsRef<Path>,
) -> String
{
	// TODO fallback to `absolute`?
	let relative = pathdiff::diff_paths(absolute, base)
		.expect("absolute and base path always have common ancestor");

	let slashed = path_slash::PathBufExt::to_slash(&relative)
		.expect("Squarkdown only supports Unicode paths");

	slashed.into_owned()
}

/// Split `path` into `(folder, filename)` (for separate highlighting in messages).
pub fn split_path(path: impl AsRef<Path>) -> (String, String)
{
	let path = path.as_ref();

	let folder = path.parent()
		.expect("cannot split a path without a parent folder");
		
	let filename = path.file_name()
		.expect("path always has a file name")
		.to_string_lossy()
		.to_string();

	(slash!("{}", folder), filename)
}

/// Split `path` into `(folder_rel, filename)` relative to `base` (for separate highlighting in messages).
pub fn split_path_from(path: impl AsRef<Path>, base: impl AsRef<Path>) -> (String, String)
{
	split_path(
		path.as_ref().map_or_orig(|p| p.strip_prefix(base).ok())
	)
}

/// Read all of the folders under `path`, and apply `f` to each.
pub fn for_folders(
	path: impl AsRef<Path>,
	mut f: impl FnMut(fs::DirEntry),
) -> io::Result<()>
{
	for dir in fs::read_dir(path)? {
		let dir = dir?;

		if let Ok(file_type) = dir.file_type()
			&& file_type.is_dir()
		{
			f(dir);
		}
	}
	Ok(())
}
