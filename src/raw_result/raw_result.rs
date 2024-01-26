use std::collections::{HashMap, HashSet};

use derive_getters::Getters;

use crate::competition::competition::Competition;
use crate::competition::competition_result::CompetitionResult;
use crate::convention::convention::Convention;
use crate::load_raw_results;
use crate::result::age_group::AgeGroup;
use crate::result::place::Place;
use crate::result::result_type::ResultType;
use crate::result::result_value::ResultValue;
use crate::utils::DATA_FOLDER;

#[derive(Getters)]
pub struct RawResult {
    ids: String,
    name: String,
    gender: String,
    age: u8,
    competition: String,
    place: String,
    result_type: String,
    result: String,
    details: String,
    age_group: String,
}

impl RawResult {
    pub fn new(
        ids: String,
        name: String,
        gender: String,
        age: u8,
        competition: String,
        place: String,
        result_type: String,
        result: String,
        details: String,
        age_group: String,
    ) -> Self {
        Self { ids, name, gender, age, competition, place, result_type, result, details, age_group }
    }
}

pub fn load_raw_results_for_conventions(conventions: &HashSet<Convention>) -> HashMap<&Convention, Vec<RawResult>> {
    let mut results = HashMap::new();

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
        results.insert(convention, raw_results);
    }


    results
}

pub fn get_results_from_raw_results_lines(raw_results: &Vec<RawResult>) -> Vec<CompetitionResult> {
    let mut results = vec![];

    for raw_result in raw_results {
        let result = match read_competition_result_from_raw_result(raw_result) {
            Ok(result) => { result }
            Err(error) => {
                eprintln!("Can't read raw result line: {}", error);
                continue;
            }
        };
        results.extend(result);
    }

    results
}

fn get_single_id(id: &str) -> Option<u16> {
    match id.parse::<u16>() {
        Ok(id) => Some(id),
        Err(_) => { None }
    }
}

fn get_ids_from_raw_result(ids: &str) -> Vec<u16> {
    let ids = ids.replace(" ", "");
    let ids: Vec<&str> = ids
        .split(',')
        .collect();

    let mut ids_vec = vec![];

    for id in &ids {
        match id.parse::<u16>() {
            Ok(id) => ids_vec.push(id),
            Err(_) => {
                eprintln!("Expected ID as integer, but got something else [ids: {:?}, wrong_id: {}]",
                          ids, id);
                continue;
            }
        };
    }

    ids_vec
}

fn read_competition_result_from_raw_result(raw_result: &RawResult) -> Result<Vec<CompetitionResult>, String> {
    let competition = Competition::new(raw_result.competition());
    let place = Place::from_string(raw_result.place())?;
    let result_type = ResultType::from_string(raw_result.result_type())?;
    let result = if raw_result.result().is_empty() {
        None
    } else {
        Some(ResultValue::from_string(raw_result.result()))
    };
    let details = if raw_result.details().is_empty() {
        None
    } else {
        Some(raw_result.details().clone())
    };
    let age_group = if raw_result.age_group().is_empty() {
        None
    } else {
        Some(AgeGroup::from_string(raw_result.age_group()))
    };

    let ids = get_ids_from_raw_result(raw_result.ids());
    Ok(ids.iter()
        .map(|id| CompetitionResult::new(*id, competition.clone(), place.clone(), result_type.clone(), result.clone(), details.clone(), age_group.clone()))
        .collect())
}
