use derive_getters::Getters;

use crate::competition::competition::Competition;
use crate::result::age_group::AgeGroup;
use crate::result::place::Place;
use crate::result::result_type::ResultType;
use crate::result::result_value::ResultValue;

/// A [CompetitionResult] is defined by a competition, a place,
/// a result type, a result, optional details and an age group.
#[derive(Debug, Clone, PartialEq, Getters)]
pub struct CompetitionResult {
    id: u16,
    competition: Competition,
    place: Place,
    result_type: ResultType,
    result: Option<ResultValue>,
    details: Option<String>,
    age_group: Option<AgeGroup>,
}

impl CompetitionResult {
    pub fn new(
        id: u16,
        competition: Competition,
        place: Place,
        result_type: ResultType,
        result: Option<ResultValue>,
        details: Option<String>,
        age_group: Option<AgeGroup>,
    ) -> Self {
        Self {
            id,
            competition,
            place,
            result_type,
            result,
            details,
            age_group,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::competition::competition::Competition;
    use crate::competition::competition_result::CompetitionResult;
    use crate::result::age_group::AgeGroup;
    use crate::result::place::Place;
    use crate::result::result_type::ResultType;
    use crate::result::result_value::ResultValue;

    impl CompetitionResult {
        pub fn create_test_instance() -> Self {
            CompetitionResult::new(
                1,
                Competition::new("Competition"),
                Place::from_string("1").unwrap(),
                ResultType::from_string("Overall").unwrap(),
                Some(ResultValue::from_string("00:14:99")),
                None,
                Some(AgeGroup::from_string("Senior")),
            )
        }
    }
}