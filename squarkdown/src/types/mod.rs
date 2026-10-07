//! This module implements common types shared throughout Squarkdown.

mod squark_error;
pub use squark_error::{ SquarkResult, SquarkError, CollectSquark };

mod page_data;
pub use page_data::{ PageData };

mod cleanse;
pub use cleanse::{ CleanseOperation };

mod site_data;
pub use site_data::{ SiteData, SiteStats };

mod context_stack;
pub use context_stack::{ ContextStack };


use tinyvec::TinyVec;

/// A single string on the stack, or many strings on the heap.
pub(crate) type Strings = TinyVec<[String; 1]>;
