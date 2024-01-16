use derive_getters::Getters;

#[derive(Debug, Getters, PartialEq, Eq, Hash, Clone)]
pub struct CompetitorName {
    name_parts: Vec<String>,
}

impl CompetitorName {
    pub fn new(name: String) -> Self {
        let mut name_parts: Vec<String> = name.split(' ').map(|s| s.to_string()).collect();
        name_parts.sort();
        Self { name_parts }
    }
}