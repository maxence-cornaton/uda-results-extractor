use crate::result::age_group::AgeGroup;
use crate::result::place::Place;
use crate::result::result_type::ResultType;
use crate::result::result_value::ResultValue;

/// A [CompetitionResult] is defined by a competition, a place,
/// a result type, a result, optional details and an age group.
#[derive(Debug, Clone)]
pub struct CompetitionResult {
    place: Place,
    result_type: ResultType,
    result: ResultValue,
    details: String,
    age_group: AgeGroup,
}

impl CompetitionResult {
    pub fn new(
        place: Place,
        result_type: ResultType,
        result: ResultValue,
        details: &str,
        age_group: AgeGroup,
    ) -> Self {
        Self {
            place,
            result_type,
            result,
            details: String::from(details),
            age_group,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::competition::competition_result::CompetitionResult;
    use crate::result::age_group::AgeGroup;
    use crate::result::place::Place;
    use crate::result::result_type::ResultType;
    use crate::result::result_value::ResultValue;

    impl CompetitionResult {
        pub fn create_test_instance() -> Self {
            CompetitionResult::new(
                Place::from_string("1").unwrap(),
                ResultType::from_string("Overall").unwrap(),
                ResultValue::from_string("00:14:99"),
                "",
                AgeGroup::from_string("Senior"),
            )
        }
    }
}