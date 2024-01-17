use std::collections::{HashMap, HashSet};

use derive_getters::Getters;

use crate::competition::competition::Competition;
use crate::competition::competition_result::CompetitionResult;
use crate::competitor::competitor::Competitor;
use crate::competitor::gender::Gender;
use crate::competitor::individual_competitor::IndividualCompetitor;
use crate::competitor::unknown_individual_competitor::UnknownIndividualCompetitor;
use crate::convention::convention::Convention;
use crate::registration::registration::Registration;
use crate::result::age_group::AgeGroup;
use crate::result::place::Place;
use crate::result::result_type::ResultType;
use crate::result::result_value::ResultValue;

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

pub fn read_registrations_from_raw_results_lines(convention: &Convention, raw_results: &Vec<RawResult>) -> Vec<Registration> {
    let mut registrations = HashMap::new();

    let mut known_competitions = HashSet::new();

    for raw_result in raw_results {
        let result = read_raw_result_line(raw_result);
        if result.is_err() {
            eprintln!("Can't create registration based on raw result line: {}", result.unwrap_err());
            continue;
        }

        let (competitors, competition, competition_result) = result.unwrap();

        // Deduplicate competitions => if competition already known, then reuse the old one.
        known_competitions.insert(competition.clone());
        let competition = known_competitions.get(&competition).unwrap();

        for (id, competitor) in competitors {
            let registration = registrations.get_mut(&id);
            if registration.is_none() {
                let mut results = HashMap::new();
                results.insert(competition.clone(), vec![competition_result.clone()]);
                let registration = Registration::new(competitor, convention.clone(), results);
                registrations.insert(id, registration);
            } else {
                let mut registration = registration.unwrap();
                registration.add_result(competition.clone(), competition_result.clone());
            }
        }
    }

    let mut registrations_as_vec = vec![];

    for (_, registration) in registrations {
        registrations_as_vec.push(registration.clone());
    }

    registrations_as_vec
}

fn read_raw_result_line(raw_result: &RawResult) -> Result<(HashMap<u16, Competitor>, Competition, CompetitionResult), String> {
    let competitors = read_competitors_from_raw_result(raw_result);
    let competition = Competition::new(raw_result.competition().clone());
    let competition_result = match read_competition_result_from_raw_result(raw_result) {
        Ok(competition_result) => { competition_result }
        Err(error) => {
            return Err(format!("Can't read raw result line: {}", error));
        }
    };

    Ok((competitors, competition, competition_result))
}

/// Creates [Competitor]s indexed by their IDs from a [RawResult] line.
fn read_competitors_from_raw_result(raw_result: &RawResult) -> HashMap<u16, Competitor> {
    let mut competitors = HashMap::new();

    let id = get_single_id(raw_result.ids());
    if id.is_some() {
        let id = id.unwrap();

        let gender = match Gender::from_string(raw_result.gender()) {
            Ok(gender) => { gender }
            Err(error) => {
                eprintln!("{}", error);
                return competitors;
            }
        };

        let competitor = IndividualCompetitor::new(
            id,
            raw_result.name().clone(),
            gender,
            *raw_result.age(),
        );
        competitors.insert(id, Competitor::IndividualCompetitor(competitor));
    } else {
        // List of competitors: team sport (uni-basketball, uni-hockey, pair or group freestyle)
        // FIXME: handle [Team]s
        let (ids, errors) = split_ids(raw_result.ids());
        for error in errors {
            eprintln!("{}", error);
        }
        if !ids.is_empty() {
            for id in ids {
                let competitor = UnknownIndividualCompetitor::new(
                    id
                );
                competitors.insert(id, Competitor::UnknownIndividualCompetitor(competitor));
            }
        }
    }

    competitors
}

fn get_single_id(id: &str) -> Option<u16> {
    match id.parse::<u16>() {
        Ok(id) => Some(id),
        Err(_) => { None }
    }
}

fn split_ids(ids: &str) -> (Vec<u16>, Vec<String>) {
    let ids = ids.replace(" ", "");
    let ids: Vec<&str> = ids
        .split(',')
        .collect();

    let mut ids_vec = vec![];
    let mut errors = vec![];

    for id in &ids {
        match id.parse::<u16>() {
            Ok(id) => ids_vec.push(id),
            Err(_) => {
                let error_message = format!("Expected ID as integer, but got something else [ids: {:?}, wrong_id: {}]",
                                            ids, id);
                errors.push(error_message);
            }
        };
    }

    (ids_vec, errors)
}

fn read_competition_result_from_raw_result(raw_result: &RawResult) -> Result<CompetitionResult, String> {
    let place = Place::from_string(raw_result.place())?;
    let result_type = ResultType::from_string(raw_result.result_type())?;
    let result = ResultValue::Some(raw_result.result().clone());
    let details = raw_result.details();
    let age_group = AgeGroup::from_string(raw_result.age_group());

    Ok(CompetitionResult::new(place, result_type, result, details, age_group))
}
