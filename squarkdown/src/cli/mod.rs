//! This module implements the commands Squarkdown has.
//! 
//! (Not that it has many, [`squarkdown`] (and [`init`] in future) are the only 2 that really matter.)

mod squarkup; pub use squarkup::squarkdown;
mod help;     pub use help::help;
