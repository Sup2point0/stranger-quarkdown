#[macro_export]
macro_rules! str {
	($t:expr) => {
		String::from($t)
	};
}

pub(crate) use str;


#[macro_export]
macro_rules! dir {
	($base:ident $(/ $part:expr)*) => {
		$base$(.join($part))*
	};
}

pub(crate) use dir;
