use std::collections::HashMap;

use derive_getters::Getters;

use crate::convention::convention::Convention;
use crate::person::identity::Identity;
use crate::person::person_name::PersonName;
use crate::registration::registrant::Registrant;
use crate::registration::registration::Registration;

#[derive(Clone, Debug, Getters)]
pub struct Person<'a> {
    identity: Identity,
    registered: HashMap<&'a Convention, u16>,
    registrations: Vec<Registration>,
}

impl<'a> Person<'a> {
    pub fn new(identity: Identity, registered: HashMap<&'a Convention, u16>) -> Self {
        let registrations = vec![];
        Self { identity, registered, registrations }
    }

    pub fn add_registration(&mut self, registration: Registration) {
        self.registrations.push(registration);
    }
}

pub fn create_people_from_registrants<'a>(registrants: &HashMap<&'a Convention, Vec<Registrant>>) -> Vec<Person<'a>> {
    let mut people_information: HashMap<Identity, HashMap<&Convention, u16>> = HashMap::new();

    for (convention, registrants) in registrants {
        for registrant in registrants {
            let name = PersonName::from_names(&[registrant.first_name(), registrant.last_name()]);
            let birthday = registrant.birthday();
            let identity = Identity::new(name, *birthday);

            let mut option = people_information.get_mut(&identity);
            if option.is_some() {
                option.as_mut().unwrap().insert(*convention, *registrant.id());
            } else {
                let mut information = HashMap::new();
                information.insert(*convention, *registrant.id());
                people_information.insert(identity, information);
            }
        }
    }

    let mut people = vec![];
    for (identity, registered) in people_information {
        people.push(Person::new(identity, registered));
    }

    people
}