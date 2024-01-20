use chrono::NaiveDate;
use derive_getters::Getters;

use crate::person::person_name::PersonName;

/// The identity of a person is defined by their names and their birthday.
#[derive(Clone, Debug, Getters, PartialEq, Eq, Hash)]
pub struct Identity {
    person_name: PersonName,
    birthday: NaiveDate,
    // FIXME: add support for country
}

impl Identity {
    pub fn new(person_name: PersonName, birthday: NaiveDate) -> Self {
        Self { person_name, birthday }
    }
}
