use std::collections::HashMap;

use crate::competition_result::CompetitionResult;
use crate::competitor_name::CompetitorName;
use crate::convention::Convention;

/// A [Competitor] is someone that takes part in one or more conventions.
pub struct Competitor {
    name: CompetitorName,
    /// Maps a convention to a competitor ID
    conventions: HashMap<Convention, u16>,
    results: Vec<CompetitionResult>,
}

impl Competitor {
    pub fn new(
        name: CompetitorName,
        conventions: HashMap<Convention, u16>,
        results: Vec<CompetitionResult>,
    ) -> Self {
        Self { name, conventions, results }
    }
}