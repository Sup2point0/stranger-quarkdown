#[derive(Clone, Debug, PartialEq, Eq)]
pub enum FileError
{
	MissingField {
		field: String,
	},

	InvalidValue {
		field: String,
		value: String,
	},
}

impl std::error::Error for FileError {}

impl std::fmt::Display for FileError
{
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
	{
		match self
		{
			Self::MissingField { field } => write!(f, "missing field: {field}"),
			Self::InvalidValue { field, value } => write!(f, "invalid value: {value}, for field: {field}"),
		}
	}
}
