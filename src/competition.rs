use crate::convention::Convention;

#[derive(Debug, Clone)]
pub struct Competition {
    convention: Convention,
    name: String,
}

impl Competition {
    pub fn new(convention: Convention, name: String) -> Self {
        Self { convention, name }
    }
}