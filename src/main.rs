use std::collections::HashMap;

use calamine::{Error, open_workbook, RangeDeserializerBuilder, Reader, Xls};

use crate::competitor::competitor::Competitor;
use crate::convention::convention::Convention;
use crate::person::person::{create_people_from_registrations, Person};
use crate::person::person_name::PersonName;
use crate::raw_result::raw_result::{RawResult, read_registrations_from_raw_results_lines};
use crate::registration::registration::Registration;

mod competition;
mod convention;
mod result;
mod person;
mod competitor;
mod registration;
mod raw_result;

fn main() {
    let mut conventions = HashMap::new();
    conventions.insert("CFM 2023", "cfm2023.xls");
    conventions.insert("Unicon 20", "unicon20.xls");

    let mut all_registrations = vec![];
    for (convention_name, file_name) in conventions {
        let convention = Convention::new(String::from(convention_name));
        let raw_results = match load_raw_results(file_name) {
            Ok(raw_results) => { raw_results }
            Err(error) => {
                eprintln!("Can't load raw results: {}", error);
                continue;
            }
        };
        let mut registrations = read_registrations_from_raw_results_lines(&convention, &raw_results);
        all_registrations.append(&mut registrations);
    }

    let people = create_people_from_registrations(&all_registrations);
    filter_people_on_name(&people, "Maxence Cornaton");
}

fn load_raw_results(file_name: &str) -> Result<Vec<RawResult>, Error> {
    let path = format!("{}/resources/{}", env!("CARGO_MANIFEST_DIR"), file_name);
    let mut workbook: Xls<_> = open_workbook(path)?;
    let range = workbook.worksheet_range("Worksheet1")
        .map_err(|_error| Error::Msg("Cannot find 'Worksheet1'"))?;

    let mut iter = RangeDeserializerBuilder::new().from_range(&range)?;

    let mut raw_results: Vec<RawResult> = vec![];

    while let Some(result) = iter.next() {
        let (id, name, gender, age, competition, place, result_type, result, details, age_group): (String, String, String, u8, String, String, String, String, String, String) = result?;
        let raw_result = RawResult::new(id, name, gender, age, competition, place, result_type, result, details, age_group);
        raw_results.push(raw_result);
    }

    Ok(raw_results)
}

fn filter_registrations_on_competitor_name(registrations: &Vec<Registration>, competitor_name: &str) {
    for registration in registrations {
        match registration.competitor() {
            Competitor::IndividualCompetitor(competitor) => {
                if competitor.name().to_lowercase() == competitor_name.to_lowercase() {
                    println!("{:?}", registration);
                }
            }
            Competitor::Team(_) => {}
            Competitor::UnknownIndividualCompetitor(_) => {}
        }
    }
}

fn filter_registrations_on_id(registrations: &Vec<Registration>, id: &u16) {
    for registration in registrations {
        match registration.competitor() {
            Competitor::IndividualCompetitor(competitor) => {
                if competitor.id() == id {
                    println!("{:?}", registration);
                }
            }
            Competitor::Team(_) => {}
            Competitor::UnknownIndividualCompetitor(competitor) => {
                if competitor.id() == id {
                    println!("{:?}", registration);
                }
            }
        }
    }
}

fn filter_people_on_name(people: &Vec<Person>, name: &str) {
    let name = PersonName::new(name);
    for person in people {
        if person.name() == &name {
            println!("{} => {:?}", person.name().name(), person.registrations());
        }
    }
}