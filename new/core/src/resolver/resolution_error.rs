use crate::utils::colours::*;


#[derive(Clone, Debug, PartialEq, thiserror::Error)]
pub enum ResolutionError
{
	#[error("could not find a parent directory with a {}.squarkdown{} folder", BLUE, RED)]
	NoSquarkdown,
}
