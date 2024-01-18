use std::fs::create_dir_all;

pub mod credentials;
pub mod env_manager;

pub const DATA_FOLDER: &str = "data";

pub fn create_folder(path: &str, error_message: &str) -> Result<(), ()> {
    create_dir_all(path).or_else(
        |error| {
            eprintln!("{}", error_message);
            eprintln!("{error}");
            Err(())
        }
    )
}
