use thiserror::Error;


pub type RenderResult = Result<(), RenderError>;


#[derive(Clone, Debug, Error)]
pub enum RenderError
{}
