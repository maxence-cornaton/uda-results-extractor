use std::collections::HashMap;

use derive_getters::Getters;

use crate::competitor::competitor::Competitor;
use crate::person::person_name::PersonName;
use crate::registration::registration::Registration;

#[derive(Clone, Debug, Getters)]
pub struct Person {
    name: PersonName,
    registrations: Vec<Registration>,
}

impl Person {
    pub fn new(name: PersonName) -> Self {
        Self {
            name,
            registrations: vec![],
        }
    }

    pub fn add_registration(&mut self, registration: Registration) {
        self.registrations.push(registration);
    }
}

pub fn create_people_from_registrations(registrations: &Vec<Registration>) -> Vec<Person> {
    let mut people = HashMap::new();

    for registration in registrations {
        match registration.competitor() {
            Competitor::IndividualCompetitor(competitor) => {
                let competitor_name = competitor.name();
                let person_name = PersonName::new(competitor_name);
                let option = people.get_mut(&person_name);
                if option.is_none() {
                    let mut person = Person::new(person_name.clone());
                    person.add_registration(registration.clone());
                    people.insert(person_name, person);
                } else {
                    let person = option.unwrap();
                    person.add_registration(registration.clone());
                }
            }
            Competitor::Team(_) => {}
            Competitor::UnknownIndividualCompetitor(_) => {}
        }
    }

    let mut people_as_vec = vec![];
    for (_, person) in people {
        people_as_vec.push(person.clone());
    }

    people_as_vec
}