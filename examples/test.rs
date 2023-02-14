//! DESL (Despero Engine Shader Language)

use desl::Desl;

fn main() {
    let my_shader = desl::compile!("test0.desl");
        
    println!("Vertex: {:?}\n\nFragment: {:?}\n\nSource: {}", 
        my_shader.vertex(),
        my_shader.fragment(),
        my_shader.source(),
    );
}
