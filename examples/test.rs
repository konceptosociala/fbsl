//! SOL (Sonja Ombrigila Lingvo)

use sol_rs::prelude::*;

fn main() {
    let my_shader = sol_rs::compile!("test.sol");
        
    println!("{my_shader:#?}");
}
