mod config; pub use config::*;
mod file_data; pub use file_data::*;
mod file_error; pub use file_error::*;


use tinyvec::{ TinyVec, tiny_vec };

pub type Strings = TinyVec<[String; 4]>;

#[macro_export]
macro_rules! strings {
	() => { tiny_vec!([String; 4]) }
}
