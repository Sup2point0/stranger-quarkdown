//! The Squarkdown core engine, containing all of the components required for squarkup. Assembling them together into the full squarkup pipeline is left to `main.rs`.
//! 
//! # Quickstart
//! 
//! If you’re looking to understand how Squarkdown works, the best place to start is [`cli::squarkdown()`], the entry point to the squarkup pipeline. It uses the [`resolver`] to find paths and files, loads the user’s [`SquarkupConfig`](config::SquarkupConfig), then uses the [`parser`] to parse [`PageData`](types::PageData) from charm squarks, the [`renderer`] to output `+page.svx` and `+page.ts` files, and then saves [`SiteData`](types::SiteData).
//! 
//! Throughout Squarkdown I use a lot of shorthands designed to be as compact as possible, which you’ll find in the [`utils`] modules, lifted to [`log`], [`colours`], [`macros`].
//! 
//! > If you’re reading this on docs.rs, note that Squarkdown is not intended to be a library, and all of this is unstable implementation detail. (It’s published on docs.rs because Squarkdown uses a `lib.rs`+`main.rs` structure, and docs.rs always builds the library docs for a crate.)
//! 
//! # Error Handling
//! 
//! Squarkdown has somewhat complex error handling using [`SquarkError`](types::SquarkError). The complexity stems from 2 needs:
//! 
//! - Behaviour depends on `errors.on-error` in [`SquarkupConfig`](config::SquarkupConfig). When set to [`WARN`](config::ErrorAction::WARN) we print errors and continue; when set to [`KILL`](config::ErrorAction::KILL) we crash.
//! - We aggregate error messages so that we can report more than one issue to the user at a time.
//! 
//! At aggregation boundaries we use [`SquarkError::Multiple`](types::SquarkError::Multiple) with its [`.push()`](types::SquarkError::push) method (accessed through [`catch!`]) to accumulate errors while performing an operation.

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
