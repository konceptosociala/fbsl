//! FBSL (Flatbox Shader Language)

use fbsl::prelude::*;

fn main() {
    let my_shader = fbsl::compile!("test.fbsl");
        
    println!("{my_shader:#?}");
}
