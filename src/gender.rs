const AUTHORIZED_GENDER: [&str; 2] = ["Male", "Female"];

#[derive(Debug, Clone)]
pub struct Gender {
    gender: String,
}

impl Gender {
    pub fn from_string(gender: &str) -> Result<Gender, String> {
        let gender = gender.to_lowercase();
        let authorized_gender = AUTHORIZED_GENDER.iter().find(|p| p.to_string().to_lowercase() == gender);
        if authorized_gender.is_some() {
            return Ok(Gender { gender: authorized_gender.copied().unwrap().to_string() });
        }

        Err(format!("Unknown gender [gender: {}]", gender))
    }
}