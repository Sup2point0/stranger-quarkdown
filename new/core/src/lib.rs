#![allow(unused_doc_comments)]

pub mod parser;

pub mod types;
pub mod utils;

/* These types are central to all of Squarkdown, so we'll root-export them for convenience. */
pub use types::{
	SquarkupConfig, FileData,
};
