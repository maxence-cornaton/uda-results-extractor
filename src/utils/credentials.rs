use std::env;
use std::fs::File;
use std::io::Read;

use derive_getters::Getters;

use crate::utils::retrieve_env_value_from_line;

const UDA_USERNAME_ENV_VAR: &str = "UDA_USERNAME";
const UDA_PASSWORD_ENV_VAR: &str = "UDA_PASSWORD";

#[derive(Getters, Debug)]
pub struct Credentials {
    username: String,
    password: String,
}

impl Credentials {
    pub fn new(username: String, password: String) -> Self {
        Self { username, password }
    }

    pub fn load_credentials() -> Result<Credentials, String> {
        let credentials = Credentials::load_credentials_from_env_vars();
        if credentials.is_some() {
            return Ok(credentials.unwrap());
        }

        let credentials = Credentials::load_credentials_from_env_file();
        if credentials.is_some() {
            return Ok(credentials.unwrap());
        }

        Err(String::from("No credentials have been found."))
    }

    fn load_credentials_from_env_vars() -> Option<Credentials> {
        let username = match env::var(UDA_USERNAME_ENV_VAR) {
            Ok(username) => { username }
            Err(_) => { return None; }
        };
        let password = match env::var(UDA_PASSWORD_ENV_VAR) {
            Ok(password) => { password }
            Err(_) => { return None; }
        };

        Some(Credentials::new(username, password))
    }

    fn load_credentials_from_env_file() -> Option<Credentials> {
        let mut env_file = match File::open(format!("{}/.env", env!("CARGO_MANIFEST_DIR"))) {
            Ok(file) => { file }
            Err(_) => { return None; }
        };
        let mut env_file_content = String::new();
        match env_file.read_to_string(&mut env_file_content) {
            Ok(result) => { result }
            Err(_) => { return None; }
        };

        let split: Vec<&str> = env_file_content.split('\n').collect();
        let mut username = "";
        let mut password = "";
        for env_line in split {
            match retrieve_env_value_from_line(env_line, UDA_USERNAME_ENV_VAR) {
                None => {}
                Some(value) => {
                    username = value
                }
            };
            match retrieve_env_value_from_line(env_line, UDA_PASSWORD_ENV_VAR) {
                None => {}
                Some(value) => {
                    password = value
                }
            };
        }

        if !username.is_empty() && !password.is_empty() {
            Some(Credentials::new(String::from(username), String::from(password)))
        } else {
            None
        }
    }
}
