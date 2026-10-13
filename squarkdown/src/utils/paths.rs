use super::macros::slash;

use std::path::Path;


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
	path.trim_start_matches(['/', '\\'])
}

/// Display an `absolute` path relative to `base`.
#[must_use]
pub fn display_rel(
	absolute: impl AsRef<Path>,
	base: impl AsRef<Path>,
) -> String
{
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
