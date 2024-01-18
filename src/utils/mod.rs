pub mod credentials;
pub mod env_manager;

pub fn retrieve_env_value_from_line<'a>(line: &'a str, key: &str) -> Option<&'a str> {
    if line.starts_with(key) {
        let option = line.split_once('=');
        if option.is_some() {
            return Some(option.unwrap().1.trim_end_matches('\r'));
        }
    }

    None
}
