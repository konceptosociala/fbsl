//! DESL (Despero Engine Shader Language)

use desl::prelude::*;

fn main() {
    let my_shader = desl::compile!("test.desl");
        
    println!("{my_shader:#?}");
}
