pub mod log;
pub mod colours;
pub mod macros;

pub mod strings; pub use strings::*;

#[cfg(test)] pub mod testing;
#[cfg(test)] pub use testing::*;
