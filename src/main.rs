use std::collections::HashMap;

use calamine::{Error, open_workbook, RangeDeserializerBuilder, Reader, Xls};

use crate::result_entry::ResultEntry;

mod result_entry;
mod competitor_name;
mod place;
mod result_type;

fn main() {
    let filenames = vec![
        String::from("cfm2023.xls"),
        String::from("unicon20.xls"),
    ];

    let mut all_results = HashMap::new();
    for filename in filenames {
        match open_results(&filename) {
            Ok(convention_results) => { all_results.insert(filename, convention_results); }
            Err(error) => { eprintln!("{}", error); }
        };
    }

    let competitors = ResultEntry::compute_competitors(&all_results);

    println!("{:?}", competitors);
}

fn open_results(filename: &String) -> Result<Vec<ResultEntry>, Error> {
    let path = format!("{}/resources/{}", env!("CARGO_MANIFEST_DIR"), filename);
    let mut workbook: Xls<_> = open_workbook(path)?;
    let range = workbook.worksheet_range("Worksheet1")
        .map_err(|_error| Error::Msg("Cannot find 'Worksheet1'"))?;

    let mut iter = RangeDeserializerBuilder::new().from_range(&range)?;

    let mut results: Vec<ResultEntry> = vec![];

    while let Some(result) = iter.next() {
        let (id, name, gender, age, competition, place, result_type, result, details, age_group): (String, String, String, u8, String, String, String, String, String, String) = result?;
        match ResultEntry::from_result_line(&id, &name, &gender, age, &competition, &place, &result_type, &result, &details, &age_group) {
            Ok(mut result_entries) => { results.append(&mut result_entries) }
            Err(error) => { println!("Error while parsing line: {}", error) }
        };
    }

    Ok(results)
}

