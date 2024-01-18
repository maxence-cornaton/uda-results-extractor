use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::Read;

thread_local!(static ENV_FILE_VALUES: HashMap<String, String> = load_env_file() );

fn load_env_file() -> HashMap<String, String> {
    let mut env_vars = HashMap::new();

    let mut env_file = match File::open(".env".to_string()) {
        Ok(file) => { file }
        Err(_) => { return env_vars; }
    };
    let mut env_file_content = String::new();
    match env_file.read_to_string(&mut env_file_content) {
        Ok(result) => { result }
        Err(_) => { return env_vars; }
    };

    let lines: Vec<&str> = env_file_content
        .split('\n')
        .filter(|line| !line.trim().is_empty())
        .collect();
    for line in lines {
        match line.split_once('=') {
            None => {}
            Some((key, value)) => { env_vars.insert(key.to_string(), value.trim_end_matches('\r').to_string()); }
        };
    }

    env_vars
}

pub fn retrieve_env_value(key: &str) -> Option<String> {
    let option = env::var(key);
    if option.is_ok() {
        return option.ok();
    }

    ENV_FILE_VALUES.with(|values| values.get(key).cloned())
}
