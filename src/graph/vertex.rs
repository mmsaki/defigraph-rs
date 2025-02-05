#[derive(Hash, PartialEq, Eq, Debug)]
pub struct Vertex {
    pub address: String,
    pub decimals: usize,
    pub name: String,
}

impl Vertex {
    pub fn new(name: String, decimals: usize, address: String) -> Self {
        Self {
            name,
            address,
            decimals,
        }
    }
}
