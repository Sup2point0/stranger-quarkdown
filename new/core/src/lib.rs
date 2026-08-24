#![allow(unused_doc_comments)]

pub mod resolver;
pub mod parser; pub use parser::CharmParser;
pub mod renderer; pub use renderer::Renderer;

pub mod types;
pub mod errors;
pub mod utils;

/* These types are central to all of Squarkdown, so we'll root-export them for convenience. */
pub use types::{ SquarkupConfig, PageData, SiteData };
pub use errors::{ SquarkResult, SquarkError };
