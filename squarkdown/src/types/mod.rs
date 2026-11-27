//! This module implements common types shared throughout Squarkdown.

mod squark_error;
pub use squark_error::{ SquarkAction, SquarkResult, SquarkError, CollectSquark };

mod page_data;
pub use page_data::{ PageData };

mod series;
pub use series::{ Series };

mod site_data;
pub use site_data::{ SiteData, SiteStats };


mod shard;
pub use shard::{ Shard };

mod context_stack;
pub(crate) use context_stack::{ ContextStack };

mod strings;
pub(crate) use strings::{ Strings, StringsContains };
