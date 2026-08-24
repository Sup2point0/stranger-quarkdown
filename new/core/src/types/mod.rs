mod config; pub use config::*;
mod file_data; pub use file_data::*;
mod file_error; pub use file_error::*;


use tinyvec::TinyVec;

pub type Strings = TinyVec<[String; 4]>;
