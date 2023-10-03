#![allow(unused_imports)]
use std::fs;
use pest::Parser;

use crate::error::*;

#[derive(Parser)]
#[grammar = "fbsl.pest"]
pub struct FbslParser;

pub fn parse(shader: &str) -> FbslResult<&'static str> {    
    let _fbsl = match FbslParser::parse(Rule::Shader, &shader) {
        Ok(p) => p,
        Err(e) => return Err(e.into()),
    };

    println!("{_fbsl}");
    
    Ok("#version 310 es\n void main() {}")
}
