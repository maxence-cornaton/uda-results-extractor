use calamine::{Error, open_workbook, RangeDeserializerBuilder, Reader, Xls};
use derive_getters::Getters;

use crate::result_entry::ResultEntry;

#[derive(Debug, Getters)]
pub struct ConventionResults {
    name: String,
    results: Vec<ResultEntry>,
}

impl ConventionResults {
    pub fn new(name: &str, results: Vec<ResultEntry>) -> Self {
        ConventionResults {
            name: String::from(name),
            results,
        }
    }

    pub fn open_results(filename: &String) -> Result<ConventionResults, Error> {
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

        Ok(ConventionResults::new(filename, results))
    }
}