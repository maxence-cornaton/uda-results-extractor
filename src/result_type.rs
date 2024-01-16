const AUTHORIZED_RESULT_TYPES: [&str; 2] = ["AgeGroup", "Overall"];

/// A result type may only be one of [AUTHORIZED_RESULT_TYPES].
#[derive(Debug, Clone)]
pub struct ResultType {
    result_type: String,
}

impl ResultType {
    pub fn from_string(result_type: &str) -> Result<Self, String> {
        let result_type = result_type.to_lowercase();
        let authorized_result_type = AUTHORIZED_RESULT_TYPES.iter().find(|p| p.to_string().to_lowercase() == result_type);
        if authorized_result_type.is_some() {
            return Ok(ResultType { result_type: authorized_result_type.copied().unwrap().to_string() });
        }

        Err(format!("Invalid result type [result_type: {}]", result_type))
    }
}