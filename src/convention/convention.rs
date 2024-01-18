use derive_getters::Getters;

#[derive(Debug, Clone, Getters)]
pub struct Convention {
    tag: String,
    name: String,
}

impl Convention {
    pub fn new(tag: String, name: String) -> Self {
        Self { tag, name }
    }
}