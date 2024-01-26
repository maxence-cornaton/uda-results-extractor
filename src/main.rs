use std::collections::HashSet;

use calamine::{Error, open_workbook, RangeDeserializerBuilder, Reader, Xls};
use log::{error, warn};

use crate::convention::convention::{compute_conventions_to_download, Convention, dump_conventions, load_conventions_from_folder};
use crate::download::download_data;
use crate::person::person::create_people;
use crate::raw_result::raw_result::{get_results_from_raw_results_lines, load_raw_results_for_conventions, RawResult};
use crate::registration::registrant::load_registrants_for_conventions;
use crate::utils::DATA_FOLDER;
use crate::utils::env_manager::retrieve_env_value;

mod competition;
mod convention;
mod result;
mod person;
mod competitor;
mod registration;
mod raw_result;
mod download;
mod utils;

#[tokio::main]
async fn main() {
    env_logger::init();

    let conventions = match load_conventions().await {
        Ok(conventions) => { conventions }
        Err(_) => {
            error!("Aborting process");
            return;
        }
    };

    let registrants = match load_registrants_for_conventions(&conventions) {
        Ok(registrants) => { registrants }
        Err(error) => {
            error!("Registrants not loaded: {error}");
            error!("Aborting process");
            return;
        }
    };
    let raw_results = load_raw_results_for_conventions(&conventions);
    let results = raw_results.iter()
        .map(|(convention, raw_results)| (*convention, get_results_from_raw_results_lines(raw_results)))
        .collect();
    let people = create_people(&registrants, &results);

    // info!("{:?}", people);
}

async fn load_conventions() -> Result<HashSet<Convention>, ()> {
    let conventions_tag = match retrieve_env_value("CONVENTIONS") {
        None => {
            error!("No convention to deal with, can't continue...");
            return Err(());
        }
        Some(conventions_tag) => { conventions_tag.split(',').map(str::trim).map(str::to_string).collect() }
    };
    let loaded_conventions = load_conventions_from_folder(DATA_FOLDER, &conventions_tag);
    let mut conventions = HashSet::from_iter(loaded_conventions.iter().map(|(_, convention)| convention.clone()));
    let conventions_to_download = compute_conventions_to_download(&loaded_conventions, &conventions_tag);
    let downloaded_conventions = if !conventions_to_download.is_empty() {
        let data = download_data(&conventions_to_download).await;
        if data.is_err() {
            error!("No data, can't continue...");
            return Err(());
        }
        data.unwrap()
    } else {
        vec![]
    };
    conventions.extend(downloaded_conventions);

    let dump_result = dump_conventions(DATA_FOLDER, &conventions);
    if dump_result.is_err() {
        warn!("Can't dump conventions. However, process will continue.");
    }

    Ok(conventions)
}

fn load_raw_results(file_path: &str) -> Result<Vec<RawResult>, Error> {
    let mut workbook: Xls<_> = open_workbook(file_path)?;
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