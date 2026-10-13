//! This module implements the commands Squarkdown has.
//! 
//! (Not that it has many, [`squarkdown`] (and [`init`] in future) are the only 2 that really matter.)

mod help;       pub use help::help;
mod squarkdown; pub use squarkdown::squarkdown;
// mod init;       pub use init::init;
mod clean;      pub use clean::clean;

mod shared;
