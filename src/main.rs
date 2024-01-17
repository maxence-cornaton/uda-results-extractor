use std::collections::HashMap;

use calamine::{Error, open_workbook, RangeDeserializerBuilder, Reader, Xls};

use crate::convention::convention::Convention;
use crate::raw_result::raw_result::{RawResult, read_registrations_from_raw_results_lines};

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

    for (convention_name, file_name) in conventions {
        let convention = Convention::new(String::from(convention_name));
        let raw_results = match load_raw_results(file_name) {
            Ok(raw_results) => { raw_results }
            Err(error) => {
                eprintln!("Can't load raw results: {}", error);
                continue;
            }
        };
        let registrations = read_registrations_from_raw_results_lines(&convention, &raw_results);
        for registration in registrations {
            println!("{:?}", registration);
        }
    }
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
