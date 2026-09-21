#![allow(unused_doc_comments)]
#![allow(non_camel_case_types)]

pub mod config;
pub mod resolver;
pub mod parser;
pub mod renderer;

pub mod types;
pub mod errors;
pub mod utils;
pub use utils::{ log, colours, macros };

/// Central Squarkdown types packaged for convenience.
pub mod prelude
{
	pub use super::config::{ SquarkupConfig };
	pub use super::parser::{ CharmParser };
	pub use super::types::{ PageData, SiteData };
	pub use super::errors::{ SquarkResult, SquarkError };
}
