use std::path::PathBuf;


pub struct SquarkupConfig
{
	pub paths: PathsConfig,
}

struct PathsConfig
{
	pub repo: PathBuf,
	pub site: PathBuf,
	pub dest: PathBuf,
	
	pub sources: Vec<String>,
	pub exclude: Vec<String>,
}
