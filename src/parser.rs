#![allow(unused_imports)]
use std::fs;
use pest::Parser;

use crate::error::*;

#[derive(Parser)]
#[grammar = "sol.pest"]
pub struct SolParser;

pub fn parse(shader: &str) -> SolResult<&'static str> {    
    let _sol = match SolParser::parse(Rule::Shader, &shader) {
        Ok(p) => p,
        Err(e) => return Err(e.into()),
    };

    println!("{_sol}");
    
    Ok("#version 310 es\n void main() {}")
}
