//! This module handles anything related to 'searching' for things, usually involving filepaths and directory traversal.

mod resolve_root;   pub use resolve_root::{ resolve_project_root };
mod resolve_repo;   pub use resolve_repo::{ resolve_project_repo };
mod resolve_config; pub use resolve_config::{ resolve_config, find_config };
mod resolve_files;  pub use resolve_files::{ resolve_files };
mod resolve_assets; pub use resolve_assets::{ resolve_assets };
