//! SOL (Sonja Ombrigila Lingvo)

use sol::prelude::*;

fn main() {
    let my_shader = sol::compile!("test.sol");
        
    println!("{my_shader:#?}");
}
