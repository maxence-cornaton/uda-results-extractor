const AUTHORIZED_GENDER: [&str; 4] = ["Male", "Female", "(mixed)", "(n/a)"];

/// A gender may only be one of [AUTHORIZED_GENDER].
#[derive(Debug, Clone)]
pub struct Gender {
    gender: String,
}

impl Gender {
    pub fn from_string(gender: &str) -> Result<Self, String> {
        let gender = gender.to_lowercase();
        let authorized_gender = AUTHORIZED_GENDER.iter().find(|p| p.to_string().to_lowercase() == gender);
        if authorized_gender.is_some() {
            return Ok(Self { gender: authorized_gender.copied().unwrap().to_string() });
        }

        Err(format!("Invalid gender [gender: {}]", gender))
    }
}