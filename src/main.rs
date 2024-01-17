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


    // let mut all_results = vec![];
    // for filename in filenames {
    //     match ConventionResults::open_results(&filename) {
    //         Ok(convention_results) => { all_results.push(convention_results); }
    //         Err(error) => { eprintln!("{}", error); }
    //     };
    // }
    //
    // let competitors = ConventionResults::compute_competitors(&all_results);
    //
    // println!("{:?}", competitors);
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
//
// fn compute_competitors_from_raw_results(raw_results: &Vec<RawResult>) -> HashMap<u16, Competitor> {
//     let mut competitors = HashMap::new();
//     for raw_result in raw_results {
//         let new_competitors = create_new_competitors_from_raw_result(raw_result, &competitors);
//         // The following line merge the new competitors with the already-known list.
//         // A newly-known competitor replaces a previously unknown competitor.
//         competitors.extend(new_competitors);
//     }
//
//     competitors
// }
//
// /// Creates new [Competitor]s indexed by their IDs from a [RawResult] line.
// /// If a competitor is already known, then it is skipped.
// fn create_new_competitors_from_raw_result(
//     raw_result: &'a RawResult,
//     competitors: &'b HashMap<u16, Competitor<'a>>,
// )
//     -> HashMap<u16, Competitor<'a>> {
//     let mut new_competitors = HashMap::new();
//
//     let id = get_single_id(raw_result.ids());
//     if id.is_some() {
//         let id = id.unwrap();
//         match get_already_known_competitor(&competitors, &id) {
//             None => {}                  // No known competitor => create them
//             Some(_) => { return new_competitors; }    // Otherwise, continue loading competitors
//         };
//
//         let gender = match Gender::from_string(raw_result.gender()) {
//             Ok(gender) => { gender }
//             Err(error) => {
//                 eprintln!("{}", error);
//                 return new_competitors;
//             }
//         };
//
//         let competitor = IndividualCompetitor::new(
//             id,
//             raw_result.name(),
//             gender,
//             *raw_result.age(),
//         );
//         new_competitors.insert(id, Competitor::IndividualCompetitor(competitor));
//     } else {
//         // List of competitors: team sport (uni-basketball, uni-hockey, pair or group freestyle)
//         let (ids, errors) = split_ids(raw_result.ids());
//         for error in errors {
//             eprintln!("{}", error);
//         }
//         if !ids.is_empty() {
//             for id in ids {
//                 if competitors.get(&id).is_none() {
//                     let competitor = UnknownIndividualCompetitor::new(
//                         id
//                     );
//                     new_competitors.insert(id, Competitor::UnknownIndividualCompetitor(competitor));
//                 }
//             }
//         }
//     }
//
//     new_competitors
// }
//
// fn get_already_known_competitor<'a>(competitors: &'a HashMap<u16, Competitor>, id: &u16) -> Option<&'a IndividualCompetitor<'a>> {
//     let competitor = competitors.get(&id);
//     if competitor.is_none() {
//         return None;
//     }
//     let competitor = competitor.unwrap();
//     match competitor {
//         Competitor::IndividualCompetitor(known_competitor) => { Some(known_competitor) }
//         Competitor::Team(_) => { None }
//         Competitor::UnknownIndividualCompetitor(_) => { None }
//     }
// }
//
// fn get_single_id(id: &str) -> Option<u16> {
//     match id.parse::<u16>() {
//         Ok(id) => Some(id),
//         Err(_) => { None }
//     }
// }
//
// fn split_ids(ids: &str) -> (Vec<u16>, Vec<String>) {
//     let ids = ids.replace(" ", "");
//     let ids: Vec<&str> = ids
//         .split(',')
//         .collect();
//
//     let mut ids_vec = vec![];
//     let mut errors = vec![];
//
//     for id in &ids {
//         match id.parse::<u16>() {
//             Ok(id) => ids_vec.push(id),
//             Err(_) => {
//                 let error_message = format!("Expected ID as integer, but got something else [ids: {:?}, wrong_id: {}]",
//                                             ids, id);
//                 errors.push(error_message);
//             }
//         };
//     }
//
//     (ids_vec, errors)
// }
//
// // fn create_result(raw_results: &RawResult) -> CompetitionResult {
// //
// // }