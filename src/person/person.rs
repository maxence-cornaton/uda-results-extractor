use std::collections::HashMap;
use std::hash::{Hash, Hasher};

use derive_getters::Getters;

use crate::competition::competition_result::CompetitionResult;
use crate::convention::convention::Convention;
use crate::person::identity::Identity;
use crate::person::person_name::PersonName;
use crate::registration::registrant::Registrant;

#[derive(Clone, Debug, Getters)]
pub struct Person<'a> {
    identity: Identity,
    // Someone can have multiple complete or incomplete registration ids if they have registered multiple times for a convention
    registrations_id: HashMap<&'a Convention, Vec<u16>>,
    results: HashMap<&'a Convention, Vec<CompetitionResult>>,
}

impl<'a> Person<'a> {
    pub fn new(identity: Identity, registrations_id: HashMap<&'a Convention, Vec<u16>>, results: HashMap<&'a Convention, Vec<CompetitionResult>>) -> Self {
        Self { identity, registrations_id, results }
    }
}

impl<'a> PartialEq for Person<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.identity == other.identity
    }
}

impl<'a> Eq for Person<'a> {}

impl<'a> Hash for Person<'a> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.identity.hash(state);
    }
}

pub fn create_people<'a>(registrants: &HashMap<&'a Convention, Vec<Registrant>>,
                         conventions_results: &HashMap<&'a Convention, Vec<CompetitionResult>>) -> Vec<Person<'a>> {
    let mut people_information: HashMap<Identity, HashMap<&Convention, Vec<u16>>> = HashMap::new();

    for (convention, registrants) in registrants {
        for registrant in registrants {
            let name = PersonName::from_names(&[registrant.first_name(), registrant.last_name()]);
            let birthday = registrant.birthday();
            let identity = Identity::new(name, *birthday);

            let mut information = people_information.entry(identity).or_insert(HashMap::new());
            let mut ids = information.entry(*convention).or_insert(vec![]);
            ids.push(*registrant.id());
        }
    }

    let mut people = vec![];
    for (identity, registrations_id) in people_information {
        let results = get_results_from_raw_results(&registrations_id, conventions_results);
        let new_person = Person::new(identity.clone(), registrations_id, results);
        people.push(new_person);
    }

    people
}

fn get_results_from_raw_results<'a>(
    registrations_id: &HashMap<&'a Convention, Vec<u16>>,
    conventions_results: &HashMap<&'a Convention, Vec<CompetitionResult>>,
) -> HashMap<&'a Convention, Vec<CompetitionResult>> {
    let mut person_results = HashMap::new();

    for (convention, ids) in registrations_id {
        let convention_results = match conventions_results.get(convention) {
            None => { continue; }
            Some(results) => { results }
        };

        for id in ids {
            let results = convention_results.iter()
                .filter(|result| result.id() == id)
                .map(|result| result.clone())
                .collect();

            person_results.insert(*convention, results);
        }
    }

    person_results
}