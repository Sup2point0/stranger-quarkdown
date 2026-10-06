//! This module implements Squarkdown's little extras, like assets copying and Google Fonts query injection.

mod assets; pub use assets::{ prep_assets };
mod fonts;  pub use fonts::{ prep_fonts };
