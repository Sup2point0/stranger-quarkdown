//! The Squarkdown core engine, containing all of the components required for squarkup. Assembling them together into the full squarkup pipeline is left to `main.rs`.

#![allow(unused_doc_comments)]
#![allow(non_camel_case_types)]

pub mod config;
pub mod resolver;
pub mod parser;
pub mod renderer;

pub mod types;
pub mod utils;
pub use utils::{ log, colours, macros };

/// Central Squarkdown types packaged for convenience.
pub mod prelude
{
	pub use super::config::{ SquarkupConfig };
	pub use super::parser::{ CharmParser };
	pub use super::types::{ SquarkResult, SquarkError, PageData, SiteData };
}
