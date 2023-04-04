use std::fmt;
use std::num::NonZeroUsize;
use std::error::Error as StdError;
use thiserror::Error;
use pest::error::Error as PestError;
use crate::parser::Rule;

#[derive(Debug, Error)]
pub enum DeslError {
    #[error("Error parsing DESL code:\n{0}")]
    ParsingError(#[from] PestError<Rule>),
    #[error("Compilation error:\n{0}")]
    CompileError(#[from] CompileError),
}

#[derive(Clone, Debug, Hash, PartialEq)]
pub struct LineColumn(NonZeroUsize, NonZeroUsize);

impl LineColumn {
    pub fn new(line: usize, col: usize) -> Self {
        LineColumn(
            NonZeroUsize::new(line).unwrap(),
            NonZeroUsize::new(col).unwrap(),
        )
    }
}

#[derive(Clone, Debug, Hash, PartialEq)]
pub struct CompileError {
    line_col: LineColumn,
    content: String,
    message: String,
}

impl CompileError {
    pub fn new(line_col: LineColumn, content: String, message: String) -> Self {
        CompileError {
            line_col,
            content,
            message,
        }
    }
    
    fn format(&self) -> String {
        let mut spacing = String::new();
        let mut pointer = String::new();
        
        for _ in 0..self.line_col.0.to_string().len() {
            spacing.push(' ');
        }
        
        for _ in 1..self.line_col.1.get() {
            pointer.push(' ');
        }
        
        pointer.push_str("^---");
        
        format!(

"Line: {}, Column: {}
{} |
{} | {}
{} | {}
{} |

ERROR: {}
", 

self.line_col.0, 
self.line_col.1, 

spacing,

self.line_col.0,
self.content,

spacing,
pointer,

spacing,

self.message,
        
        )
    }
}

impl fmt::Display for CompileError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {        
        write!(f, "{}", self.format())
    }
}

impl StdError for CompileError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        None
    }
}

pub type DeslResult<T> = Result<T, DeslError>;
