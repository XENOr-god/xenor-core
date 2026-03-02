#[derive(Debug, Clone)]
pub struct Node {
    pub id: u64,
    pub weight: f64,
}

impl Node {
    pub fn new(id: u64, weight: f64) -> Self {
        Self { id, weight }
    }
}