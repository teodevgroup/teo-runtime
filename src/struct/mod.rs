pub mod object;

pub use object::Object;

#[derive(Debug)]
pub struct Struct {
    pub path: Vec<String>,
    pub static_functions: Vec<usize>,
    pub functions: Vec<usize>,
}

