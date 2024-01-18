use derive_getters::Getters;

use crate::utils::env_manager::retrieve_env_value;

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
        let username = match retrieve_env_value(UDA_USERNAME_ENV_VAR) {
            None => { Err("No username provided") }
            Some(username) => { Ok(username) }
        }?;
        let password = match retrieve_env_value(UDA_PASSWORD_ENV_VAR) {
            None => { Err("No password provided") }
            Some(password) => { Ok(password) }
        }?;

        Ok(Credentials::new(username, password))
    }
}
