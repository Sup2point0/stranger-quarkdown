use crate::utils::colours::*;


pub type ResolutionResult<T = ()> = Result<T, ResolutionError>;

#[derive(Clone, Debug, PartialEq, thiserror::Error)]
pub enum ResolutionError
{
	#[error("failed to read from {}{target}", BLUE)]
	ReadError { target: String },

	#[error("could not find a parent directory with a {}.squarkdown{} folder", BLUE, RED)]
	NoSquarkdown,

	#[error("could not find your {}squarkup.json{} or {}squarkup.toml{}", BLUE, RED, BLUE, RED)]
	NoConfig,
}
