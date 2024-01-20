use std::collections::{HashMap, HashSet};

use chrono::NaiveDate;
use derive_getters::Getters;
use serde::Deserialize;

use crate::convention::convention::Convention;
use crate::utils::DATA_FOLDER;

#[derive(Debug, Deserialize, Getters)]
pub struct Registrant {
    #[serde(alias = "Id")]
    id: u16,
    #[serde(alias = "First Name")]
    first_name: String,
    #[serde(alias = "Last Name")]
    last_name: String,
    #[serde(alias = "Country")]
    country: String,
    #[serde(alias = "Birthday (dd/mm/yyyy)", with = "birthday_date_format")]
    birthday: NaiveDate,
}

pub fn load_registrants_for_conventions(conventions: &HashSet<Convention>) -> Result<HashMap<&Convention, Vec<Registrant>>, String> {
    let mut registrants = HashMap::new();
    for convention in conventions {
        let new_people = load_registrants_from_file(&format!("{}/{}/registrants.csv", DATA_FOLDER, convention.tag()))?;
        registrants.insert(convention, new_people);
    }

    Ok(registrants)
}

fn load_registrants_from_file(path: &str) -> Result<Vec<Registrant>, String> {
    let mut reader = csv::Reader::from_path(path)
        .or_else(|error| Err(format!("Can't read registrants file [filepath: {path}]: {error}")))?;

    let registrants = reader.deserialize()
        .map(|registrant| parse_registrant(registrant))
        .filter(Option::is_some)
        .map(Option::unwrap)
        .collect();

    Ok(registrants)
}

fn parse_registrant(registrant: Result<Registrant, csv::Error>) -> Option<Registrant> {
    let registrant: Registrant = match registrant {
        Ok(person) => { person }
        Err(error) => {
            eprintln!("Can't read registrant");
            eprintln!("{}", error);
            return None;
        }
    };
    println!("{:?}", registrant);
    Some(registrant)
}

mod birthday_date_format {
    use chrono::{DateTime, NaiveDate, Utc};
    use serde::{self, Deserialize, Deserializer, Serializer};

    const FORMAT: &'static str = "%d/%m/%Y";

    pub fn serialize<S>(
        date: &DateTime<Utc>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let s = format!("{}", date.format(FORMAT));
        serializer.serialize_str(&s)
    }

    pub fn deserialize<'de, D>(
        deserializer: D,
    ) -> Result<NaiveDate, D::Error>
        where
            D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let dt = NaiveDate::parse_from_str(&s, FORMAT).map_err(serde::de::Error::custom)?;
        Ok(dt)
    }
}