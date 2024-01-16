use std::collections::{HashMap, HashSet};

use derive_getters::Getters;

use crate::competitor_name::CompetitorName;
use crate::gender::Gender;
use crate::place::Place;
use crate::result_type::ResultType;

#[derive(Debug, Getters)]
pub struct ResultEntry {
    id: u16,
    name: CompetitorName,
    gender: Gender,
    age: u8,
    competition: String,
    place: Place,
    result_type: ResultType,
    result: String,
    details: String,
    age_group: String,
}

impl ResultEntry {
    fn new(
        id: u16,
        name: CompetitorName,
        gender: Gender,
        age: u8,
        competition: String,
        place: Place,
        result_type: ResultType,
        result: String,
        details: String,
        age_group: String,
    ) -> ResultEntry {
        ResultEntry { id, name, gender, age, competition, place, result_type, result, details, age_group }
    }

    pub fn from_result_line(
        ids: &str,
        names: &str,
        gender: &str,
        age: u8,
        competition: &str,
        place: &str,
        result_type: &str,
        result: &str,
        details: &str,
        age_group: &str,
    ) -> Result<Vec<ResultEntry>, String> {
        let gender = Gender::from_string(gender)?;
        let place = Place::from_string(place)?;
        let result_type = ResultType::from_string(result_type)?;

        // Split ids as &str into list of ids
        let ids = ids.replace(" ", "");
        let ids: Vec<&str> = ids
            .split(',')
            .collect();

        // Split names as &str into list of names
        // /!\ Each entry can be a single name, a comma-separated list of names or a team name
        let names: Vec<String> = names
            .split(" - ")
            .map(|s| String::from(s.trim()))
            .collect();

        let ids_count = ids.len();
        let names_count = names.len();
        if ids_count != names_count {
            let error_message = format!(
                "Invalid line, different count of ids and names [ids: {:?}, names: {:?}]", ids, names
            );
            return Err(String::from(error_message));
        }

        let mut result_entries = vec![];
        for i in 0..names_count {
            let id = match ids.get(i).unwrap().parse::<u16>() {
                Ok(id) => id,
                Err(_) => {
                    let error_message = format!("Expected ID as integer, but got something else [ids: {:?}, names: {:?}, wrong_id: {}]",
                                                ids, names, ids[i]);
                    return Err(String::from(error_message));
                }
            };
            let name = names.get(i).unwrap();
            let name = CompetitorName::new(name.to_string());

            result_entries.push(ResultEntry {
                id,
                name,
                gender: gender.clone(),
                age,
                competition: String::from(competition),
                place: place.clone(),
                result_type: result_type.clone(),
                result: String::from(result),
                details: String::from(details),
                age_group: String::from(age_group),
            });
        }

        Ok(result_entries)
    }

    pub fn compute_competitors(results: &HashMap<String, Vec<ResultEntry>>) -> HashSet<&CompetitorName> {
        let mut competitors = HashSet::new();
        for (_, entries) in results {
            for entry in entries {
                competitors.insert(entry.name());
            }
        }

        competitors
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::competitor_name::CompetitorName;
    use crate::gender::Gender;
    use crate::place::Place;
    use crate::result_entry::ResultEntry;
    use crate::result_type::ResultType;

    fn create_result_entry(name: &str) -> ResultEntry {
        ResultEntry::new(
            1,
            CompetitorName::new(String::from(name)),
            Gender::from_string("Male").unwrap(),
            22,
            String::from("100m"),
            Place::from_string("1").unwrap(),
            ResultType::from_string("Overall").unwrap(),
            String::from("00:14:99"),
            String::new(),
            String::from("Senior"),
        )
    }

    #[test]
    fn should_merge_same_competitor() {
        let expected_competitor_name = CompetitorName::new(String::from("John Doe"));

        let mut results = HashMap::new();
        results.insert(String::from("convention 1"), vec![create_result_entry("John Doe")]);
        results.insert(String::from("convention 2"), vec![create_result_entry("John Doe")]);

        let competitors = ResultEntry::compute_competitors(&results);
        let competitors: Vec<&CompetitorName> = competitors.into_iter().collect();
        assert_eq!(competitors, vec![&expected_competitor_name]);
    }
}