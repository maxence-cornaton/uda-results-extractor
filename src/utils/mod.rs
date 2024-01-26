use std::fs::create_dir_all;

use log::error;

pub mod credentials;
pub mod env_manager;

pub const DATA_FOLDER: &str = "data";

pub fn create_folder(path: &str, error_message: &str) -> Result<(), ()> {
    create_dir_all(path).or_else(
        |error| {
            error!("{}", error_message);
            error!("{error}");
            Err(())
        }
    )
}
