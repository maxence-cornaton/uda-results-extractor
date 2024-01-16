use calamine::Reader;

use crate::convention_results::ConventionResults;
use crate::result_entry::ResultEntry;

mod result_entry;
mod competitor_name;
mod place;
mod result_type;
mod gender;
mod convention_results;

fn main() {
    let filenames = vec![
        String::from("cfm2023.xls"),
        String::from("unicon20.xls"),
    ];

    let mut all_results = vec![];
    for filename in filenames {
        match ConventionResults::open_results(&filename) {
            Ok(convention_results) => { all_results.push(convention_results); }
            Err(error) => { eprintln!("{}", error); }
        };
    }

    let competitors = ResultEntry::compute_competitors(&all_results);

    println!("{:?}", competitors);
}
