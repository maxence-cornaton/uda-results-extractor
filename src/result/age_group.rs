#[derive(Debug, Clone)]
pub struct AgeGroup {
    groups_name: Vec<String>,
}

impl AgeGroup {
    pub fn from_string(groups_name: &str) -> Self {
        let split_groups_name = groups_name.split(" - ");
        Self { groups_name: split_groups_name.map(|s| String::from(s.trim())).collect() }
    }
}