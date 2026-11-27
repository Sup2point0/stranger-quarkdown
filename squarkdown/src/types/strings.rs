use tinyvec::TinyVec;

/// A single string on the stack, or many strings on the heap.
pub(crate) type Strings = TinyVec<[String; 1]>;


pub trait StringsContains
{
	fn contains(&self, t: impl AsRef<str>) -> bool;
}

impl StringsContains for Strings
{
	fn contains(&self, t: impl AsRef<str>) -> bool {
		self.iter().any(|flag| flag == t.as_ref())
	}
}
