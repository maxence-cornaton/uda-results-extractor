use derive_getters::Getters;

use crate::competitor::gender::Gender;

#[derive(Debug, Getters, PartialEq, Eq, Hash, Clone)]
pub struct IndividualCompetitor {
    id: u16,
    name: String,
    gender: Gender,
    age: u8,
}

impl IndividualCompetitor {
    pub fn new(id: u16, name: String, gender: Gender, age: u8) -> Self {
        Self { id, name, gender, age }
    }
}

