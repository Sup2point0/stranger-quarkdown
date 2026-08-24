use thiserror::Error;

#[derive(Clone, Debug, Error, PartialEq, Eq)]
pub enum CharmError
{
	#[error("missing field: {field}")]
	MissingField {
		field: String,
	},

	#[error("invalid value: {value}, for field: {field}")]
	InvalidValue {
		field: String,
		value: String,
	},
}
