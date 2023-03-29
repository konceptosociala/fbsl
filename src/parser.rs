#![allow(unused_imports)]
use std::fs;
use pest::Parser;

#[derive(Parser)]
#[grammar = "desl.pest"]
pub struct DeslParser;

pub fn parse(shader: &str) -> &'static str {    
    let desl = match DeslParser::parse(Rule::Shader, &shader) {
        Ok(p) => p,
        Err(r) => panic!("{}", r),
    };
    
    "#version 310 es\n void main() {}"
}
