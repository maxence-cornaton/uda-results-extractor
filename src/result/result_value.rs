/// Results may be of different types:
/// - Empty
/// - Time
/// - Distance
/// - Score
/// - Points
/// - Custom
#[derive(Debug, Clone)]
pub enum ResultValue {
    Some(String)    // FIXME
}

impl ResultValue {
    pub fn from_string(result: &str) -> Self {
        ResultValue::Some(String::from(result))
    }
}