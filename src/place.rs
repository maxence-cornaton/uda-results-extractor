const AUTHORIZED_DISQUALIFIED_STRINGS: [&str; 2] = ["DNF", "DQ"];

#[derive(Debug, Clone)]
pub enum Place {
    Rank(u8),
    Disqualified(String),
}

impl Place {
    pub fn from_string(place: &str) -> Result<Place, String> {
        match place.parse::<u8>() {
            Ok(place) => return Ok(Place::Rank(place)),
            Err(_) => {}
        };

        let place = place.to_uppercase();
        if AUTHORIZED_DISQUALIFIED_STRINGS.iter().any(|p| p.to_string() == place) {
            return Ok(Place::Disqualified(place));
        }

        Err(format!("Unknown place type [place:{}]", place))
    }
}