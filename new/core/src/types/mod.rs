mod config; pub use config::*;
mod file_data; pub use file_data::*;
mod file_error; pub use file_error::*;
mod cleanse; pub use cleanse::*;
mod site_data; pub use site_data::*;


use tinyvec::TinyVec;

pub(crate) type Strings = TinyVec<[String; 4]>;
