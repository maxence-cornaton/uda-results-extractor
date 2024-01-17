/// A competitor that takes part only to team competitions may be unknown (no name, no gender, no age)
#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub struct UnknownIndividualCompetitor {
    id: u16,
}

impl UnknownIndividualCompetitor {
    pub fn new(id: u16) -> Self {
        Self { id }
    }
}