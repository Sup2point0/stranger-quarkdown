use thiserror::Error;

/// An error encountered while processing the charm squark and initialising `PageData`.
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
