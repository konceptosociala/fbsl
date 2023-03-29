pub extern crate shaderc;
pub extern crate pest;
#[macro_use]
pub extern crate pest_derive;

pub mod error;
pub mod parser;
pub mod prelude;

#[derive(Clone, Debug, Default, Hash, PartialEq)]
pub struct Desl {
    source: String,
    vertex_compiled: Box<[u32]>,
    fragment_compiled: Box<[u32]>,
}

impl Desl {
    pub fn new(
        source: String,
        vertex_compiled: Box<[u32]>,
        fragment_compiled: Box<[u32]>,
    ) -> Self {
        Desl {
            source,
            vertex_compiled,
            fragment_compiled,
        }
    }
    
    pub fn source(&self) -> &str {
        self.source.as_str()
    }
    
    pub fn vertex(&self) -> &[u32] {
        self.vertex_compiled.as_ref()
    }
    
    pub fn fragment(&self) -> &[u32] {
        self.fragment_compiled.as_ref()
    }
}

#[macro_export]
macro_rules! compile {
    ($path:expr $(,)?) => {
        {
            use shaderc::{Compiler, CompileOptions, ShaderKind};
            
            let shader = include_str!($path);
            
            let source = parse(&shader);            
            let mut compiler = Compiler::new().unwrap();
            let mut options = CompileOptions::new().unwrap();
            
            let vertex_compiled = compiler.compile_into_spirv(
                source, 
                ShaderKind::Vertex,
                $path, 
                "main", 
                Some(&options)
            ).unwrap();
            
            let fragment_compiled = compiler.compile_into_spirv(
                source, 
                ShaderKind::Vertex,
                $path, 
                "main", 
                Some(&options)
            ).unwrap();
            
            Desl::new(
                source.into(), 
                vertex_compiled.as_binary().into(), 
                fragment_compiled.as_binary().into(),
            )
        }
    };
}
