use derive_getters::Getters;
use deunicode::deunicode;

#[derive(Debug, Getters, Eq, Hash, Clone)]
pub struct CompetitorName {
    name: String,
    name_parts: Vec<String>,
}

impl CompetitorName {
    pub fn new(name: &str) -> Self {
        let mut name_parts: Vec<String> = name
            .split(' ')
            .filter(|s| !s.is_empty())
            .map(|s| s.to_lowercase())
            .map(|s| deunicode(&s))
            .collect();
        name_parts.sort();
        Self { name: String::from(name.trim()), name_parts }
    }
}

impl PartialEq for CompetitorName {
    /// [CompetitorName]s are considered as equal if their name is strictly equal
    /// or if each part of both name is equal after accents have been replaced by their non-accentuated letters.
    fn eq(&self, other: &Self) -> bool {
        return self.name == other.name
            || self.name_parts == other.name_parts;
    }
}

#[cfg(test)]
mod tests {
    use crate::competitor_name::CompetitorName;

    #[test]
    fn should_be_equal_when_same_name() {
        let name = "John Doe";
        assert_eq!(CompetitorName::new(name), CompetitorName::new(name));
    }

    #[test]
    fn should_be_equal_when_same_name_but_more_spaces() {
        assert_eq!(CompetitorName::new("John Doe"), CompetitorName::new("  John    Doe "));
    }

    #[test]
    fn should_be_equal_when_same_name_but_reversed() {
        assert_eq!(CompetitorName::new("John Doe"), CompetitorName::new("Doe John"));
    }

    #[test]
    fn should_be_equal_when_same_name_with_accent() {
        assert_eq!(CompetitorName::new("John Doe"), CompetitorName::new("Jôhn Doé"));
    }
}