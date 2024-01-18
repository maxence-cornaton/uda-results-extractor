use std::collections::HashSet;

use calamine::{Error, open_workbook, RangeDeserializerBuilder, Reader, Xls};

use crate::competitor::competitor::Competitor;
use crate::convention::convention::{compute_conventions_to_download, dump_conventions, load_conventions};
use crate::download::download_data;
use crate::person::person::{create_people_from_registrations, Person};
use crate::person::person_name::PersonName;
use crate::raw_result::raw_result::{RawResult, read_registrations_from_raw_results_lines};
use crate::registration::registration::Registration;
use crate::utils::DATA_FOLDER;
use crate::utils::env_manager::retrieve_env_value;

mod competition;
mod convention;
mod result;
mod person;
mod competitor;
mod registration;
mod raw_result;
mod download;
mod utils;

#[tokio::main]
async fn main() {
    let conventions_tag = match retrieve_env_value("CONVENTIONS") {
        None => {
            eprintln!("No convention to deal with, can't continue...");
            return;
        }
        Some(conventions_tag) => { conventions_tag.split(',').map(str::trim).map(str::to_string).collect() }
    };
    let loaded_conventions = load_conventions(DATA_FOLDER);
    let mut conventions = HashSet::from_iter(loaded_conventions.iter().map(|(_, convention)| convention));
    let conventions_to_download = compute_conventions_to_download(&loaded_conventions, &conventions_tag);
    let downloaded_conventions = if !conventions_to_download.is_empty() {
        let data = download_data(&conventions_to_download).await;
        if data.is_err() {
            eprintln!("No data, can't continue...");
            return;
        }
        data.unwrap()
    } else {
        vec![]
    };
    conventions.extend(&downloaded_conventions);

    let dump_result = dump_conventions(DATA_FOLDER, &conventions);
    if dump_result.is_err() {
        eprintln!("Can't dump conventions. However, process will continue.");
    }

    let mut all_registrations = vec![];
    for convention in conventions {
        let file_name = format!("{}/{}/results.xls", DATA_FOLDER, convention.tag());
        let raw_results = match load_raw_results(&file_name) {
            Ok(raw_results) => { raw_results }
            Err(error) => {
                eprintln!("Can't load raw results [convention: {}, filename: {file_name}]", convention.name());
                eprintln!("{error}");
                continue;
            }
        };
        let mut registrations = read_registrations_from_raw_results_lines(&convention, &raw_results);
        all_registrations.append(&mut registrations);
    }

    let people = create_people_from_registrations(&all_registrations);
    // filter_people_on_name(&people, "Maxence Cornaton");
    // filter_people_with_asterisk(&people);
}

fn load_raw_results(file_path: &str) -> Result<Vec<RawResult>, Error> {
    let mut workbook: Xls<_> = open_workbook(file_path)?;
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

fn filter_people_with_asterisk(people: &Vec<Person>) {
    for person in people {
        if person.name().name().contains("*") {
            println!("{} => {:?}", person.name().name(), person.registrations());
        }
    }
}