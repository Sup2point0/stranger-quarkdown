//! The Squarkdown core engine, containing all of the components required for squarkup. Assembling them together into the full squarkup pipeline is left to `main.rs`.
//! 
//! If you’re reading this on docs.rs, note that Squarkdown is not intended to be a library, and all of this is unstable implementation detail. (It’s published on docs.rs because Squarkdown uses a `lib.rs`+`main.rs` structure, and docs.rs always builds the library docs for a crate.)
//! 
//! Of course, feel free to use these docs to understand the internals of how Squarkdown works!

#![allow(unused_doc_comments)]
#![allow(non_camel_case_types)]

pub mod cli;

pub mod config;
pub mod resolver;
pub mod parser;
pub mod renderer;
pub mod extras;

pub mod types;
pub mod utils;
pub use utils::{ log, colours, macros };

/// Central Squarkdown types packaged for convenience.
pub mod prelude
{
	pub use super::config::{ SquarkupConfig };
	pub use super::types::{ SquarkResult, SquarkError, PageData, SiteData };
}
