use std::collections::HashMap;

use derive_getters::Getters;

use crate::competition::competition::Competition;
use crate::competition::competition_result::CompetitionResult;
use crate::competitor::competitor::Competitor;
use crate::convention::convention::Convention;

#[derive(Clone, Debug, Getters)]
pub struct Registration {
    competitor: Competitor,
    convention: Convention,
    competitions: HashMap<Competition, Vec<CompetitionResult>>,
}

impl Registration {
    pub fn new(competitor: Competitor, convention: Convention, competitions: HashMap<Competition, Vec<CompetitionResult>>) -> Self {
        Self { competitor, convention, competitions }
    }

    pub fn add_result(self: &mut Self, competition: Competition, competition_result: CompetitionResult) {
        let known_competition = self.competitions.get_mut(&competition);
        if known_competition.is_none() {
            self.competitions.insert(competition, vec![competition_result]);
        } else {
            known_competition.unwrap().push(competition_result);
        }
    }
}