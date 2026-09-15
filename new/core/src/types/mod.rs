mod page_data; pub use page_data::*;
mod cleanse; pub use cleanse::*;
mod site_data; pub use site_data::*;


use tinyvec::TinyVec;

pub(crate) type Strings = TinyVec<[String; 4]>;
