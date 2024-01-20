use std::collections::{HashMap, HashSet};
use std::fs;
use std::fs::File;
use std::io::{BufReader, Write};

use derive_getters::Getters;
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::utils::create_folder;

const CONVENTIONS_FILE: &str = "conventions.json";

#[derive(Debug, Clone, Getters, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub struct Convention {
    tag: String,
    name: String,
}

impl Convention {
    pub fn new(tag: String, name: String) -> Self {
        Self { tag, name }
    }
}

pub fn dump_conventions(folder: &str, conventions: &HashSet<Convention>) -> Result<(), ()> {
    create_folder(
        folder,
        &format!("Can't dump conventions because folder couldn't be created [folder: {folder}]"),
    )?;
    let json = json!(conventions);
    let filepath = format!("{folder}/{CONVENTIONS_FILE}");
    let mut file = match File::create(&filepath) {
        Ok(file) => { Ok(file) }
        Err(error) => {
            eprintln!("Can't dump conventions because file couldn't be opened [filepath: {filepath}]");
            eprintln!("{}", error);
            Err(())
        }
    }?;
    match file.write_all(json.to_string().as_bytes()) {
        Ok(_) => { Ok(()) }
        Err(error) => {
            eprintln!("Can't dump conventions [filepath: {filepath}]");
            eprintln!("{}", error);
            Err(())
        }
    }
}

pub fn load_conventions_from_folder(folder: &str) -> HashMap<String, Convention> {
    let mut conventions_with_data = HashMap::new();
    let filepath = format!("{folder}/{CONVENTIONS_FILE}");
    let file = match File::open(filepath) {
        Ok(file) => { file }
        Err(_) => { return conventions_with_data; }
    };
    let reader = BufReader::new(file);

    let result: Result<HashSet<Convention>, serde_json::Error> = serde_json::from_reader(reader);
    if result.is_ok() {
        let conventions = result.unwrap();
        for convention in conventions {
            if check_convention_data_exists(folder, &convention) {
                conventions_with_data.insert(convention.tag().clone(), convention);
            }
        }
    }
    conventions_with_data
}

pub fn compute_conventions_to_download<'a>(already_downloaded_conventions: &HashMap<String, Convention>, required_conventions: &'a Vec<String>) -> HashSet<&'a String> {
    let mut conventions_to_download = HashSet::new();

    for convention_tag in required_conventions {
        if !already_downloaded_conventions.contains_key(convention_tag) {
            conventions_to_download.insert(convention_tag);
        } else {
            println!("Convention already exists locally [convention:{convention_tag}]");
        }
    }

    println!("Conventions to download: {:?}", conventions_to_download);
    conventions_to_download
}

fn check_convention_data_exists(folder: &str, convention: &Convention) -> bool {
    let mut errors = vec![];
    let results_file_path = format!("{folder}/{}/results.xls", convention.tag());
    match fs::metadata(results_file_path) {
        Ok(_) => {}
        Err(error) => { errors.push(error); }
    }

    let results_file_path = format!("{folder}/{}/results.xls", convention.tag());
    match fs::metadata(results_file_path) {
        Ok(_) => {}
        Err(error) => { errors.push(error); }
    }

    if errors.is_empty() {
        true
    } else {
        eprintln!("Convention data does not exist [convention: {}]", convention.name());
        for error in errors {
            eprintln!("{}", error);
        }
        false
    }
}