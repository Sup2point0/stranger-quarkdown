//! This module implements common types shared throughout Squarkdown.

mod squark_error;
pub use squark_error::{ SquarkResult, SquarkError };

mod page_data;
pub use page_data::{ PageData, SerialisedPageData };

mod cleanse;
pub use cleanse::{ CleanseOperation };

mod site_data;
pub use site_data::{ SiteData, SiteStats };

mod context_stack;
pub use context_stack::{ ContextStack };


use tinyvec::TinyVec;

pub(crate) type Strings = TinyVec<[String; 4]>;
