use crate::competitor::individual_competitor::IndividualCompetitor;
use crate::competitor::team::Team;
use crate::competitor::unknown_individual_competitor::UnknownIndividualCompetitor;

/// A [Competitor] is an [IndividualCompetitor], a [Team] or an [UnknownIndividualCompetitor].
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Competitor {
    IndividualCompetitor(IndividualCompetitor),
    Team(Team),
    UnknownIndividualCompetitor(UnknownIndividualCompetitor),
}