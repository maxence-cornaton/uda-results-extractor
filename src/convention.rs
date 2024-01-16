#[derive(Debug, Clone)]
pub struct Convention {
    name: String,
}

impl Convention {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}