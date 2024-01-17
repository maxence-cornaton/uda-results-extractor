const AUTHORIZED_DISQUALIFIED_STRINGS: [&str; 2] = ["DNF", "DQ"];

/// A [Place] can either be a rank denoted as an unsigned integer
/// or a disqualification acronym ("DNF", "DQ") denoted as a String.
#[derive(Debug, Clone)]
pub enum Place {
    Rank(u16),
    Disqualified(String),
}

impl Place {
    /// Tries and convert a String to a [Place].
    pub fn from_string(place: &str) -> Result<Self, String> {
        match place.parse::<u16>() {
            Ok(place) => return Ok(Self::Rank(place)),
            Err(_) => {}
        };

        let place = place.to_uppercase();
        if AUTHORIZED_DISQUALIFIED_STRINGS.iter().any(|p| p.to_string() == place) {
            return Ok(Self::Disqualified(place));
        }

        Err(format!("Invalid place type [place: {}]", place))
    }
}