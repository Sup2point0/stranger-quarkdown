#![allow(unused_doc_comments)]

pub mod config;
pub mod resolver;
pub mod parser;
pub mod renderer;

pub mod types;
pub mod errors;
pub mod utils;
pub use utils::{ log, colours, macros };

/* These types are central to all of Squarkdown, so we'll root-export them for convenience. */
pub use config::{ SquarkupConfig };
pub use parser::{ CharmParser };
pub use renderer::{ Renderer };
pub use types::{ PageData, SiteData };
pub use errors::{ SquarkResult, SquarkError };
