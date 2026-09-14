/// Make `path` relative by stripping a leading path separator.
/// 
/// This is very important to ensure `base.join(path)` remains safe.
/// 
/// For instance:
/// - `"/base/path/".join("rel/path")` resolves to `/base/path/rel/path`, happy days.
/// - `"/base/path/".join("/rel/path")` resolves to `/rel/path`.
///   - With a leading separator, the latter is treated as an absolute path, overriding the base path.
pub fn rel_path(path: &str) -> &str
{
	path.trim_start_matches(|c| matches!(c, '/' | '\\'))
}
