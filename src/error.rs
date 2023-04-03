use thiserror::Error;
use pest::error::Error as PestError;
use crate::parser::Rule;

#[derive(Debug, Error)]
pub enum DeslError {
    #[error("Error parsing DESL code:\n {0}")]
    ParsingError(#[from] PestError<Rule>),
}

pub type DeslResult<T> = Result<T, DeslError>;
