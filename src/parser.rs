#![allow(unused_imports)]
use std::fs;
use pest::Parser;

use crate::error::*;

#[derive(Parser)]
#[grammar = "desl.pest"]
pub struct DeslParser;

pub fn parse(shader: &str) -> DeslResult<&'static str> {    
    let desl = match DeslParser::parse(Rule::Shader, &shader) {
        Ok(p) => p,
        Err(e) => return Err(e.into()),
    };
    
    Ok("#version 310 es\n void main() {}")
}
