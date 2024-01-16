use std::collections::HashSet;

use calamine::{Error, open_workbook, RangeDeserializerBuilder, Reader, Xls};
use derive_getters::Getters;

use crate::competitor_name::CompetitorName;
use crate::result_entry::ResultEntry;

/// Represents all results for a convention.
/// Can be loaded through [ConventionResults::open_results].
/// Competitors of a list of conventions can be retrieved through [ConventionResults::compute_competitors].
#[derive(Debug, Getters)]
pub struct ConventionResults {
    name: String,
    results: Vec<ResultEntry>,
}

impl ConventionResults {
    /// Loads results from a file.
    /// Prints errors but they don't prevent the opening.
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
                Err(error) => { eprintln!("Error while parsing line: {}", error) }
            };
        }

        Ok(ConventionResults { name: String::from(filename), results })
    }

    /// Retrieves competitors from a list of conventions competitors,
    /// so that every competitor appears a single time.
    pub fn compute_competitors(results: &Vec<ConventionResults>) -> HashSet<&CompetitorName> {
        let mut competitors = HashSet::new();
        for entries in results {
            for entry in entries.results() {
                competitors.insert(entry.name());
            }
        }

        competitors
    }
}

#[cfg(test)]
mod tests {
    use crate::competitor_name::CompetitorName;
    use crate::convention_results::ConventionResults;
    use crate::result_entry::ResultEntry;

    #[test]
    fn should_merge_same_competitor() {
        let expected_competitor_name = CompetitorName::new(String::from("John Doe"));

        let mut results = vec![];
        results.push(ConventionResults::new("convention 1", vec![ResultEntry::create_result_entry("John Doe")]));
        results.push(ConventionResults::new("convention 2", vec![ResultEntry::create_result_entry("John Doe")]));

        let competitors = ConventionResults::compute_competitors(&results);
        let competitors: Vec<&CompetitorName> = competitors.into_iter().collect();
        assert_eq!(competitors, vec![&expected_competitor_name]);
    }
}