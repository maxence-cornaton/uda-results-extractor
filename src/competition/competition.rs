#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Competition {
    name: String,
}

impl Competition {
    pub fn new(name: &str) -> Self {
        Self { name: name.to_string() }
    }
}