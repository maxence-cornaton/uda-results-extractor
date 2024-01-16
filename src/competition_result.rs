use crate::competition::Competition;
use crate::place::Place;
use crate::result_type::ResultType;

/// A [CompetitionResult] is defined by a competition, a place,
/// a result type, a result, optional details and an age group.
#[derive(Debug)]
pub struct CompetitionResult {
    competition: Competition,
    place: Place,
    result_type: ResultType,
    result: String,
    details: String,
    age_group: String,
}

impl CompetitionResult {
    pub fn new(
        competition: Competition,
        place: Place,
        result_type: ResultType,
        result: String,
        details: String,
        age_group: String,
    ) -> Self {
        Self { competition, place, result_type, result, details, age_group }
    }
}

#[cfg(test)]
mod tests {
    use crate::competition::Competition;
    use crate::competition_result::CompetitionResult;
    use crate::convention::Convention;
    use crate::place::Place;
    use crate::result_type::ResultType;

    impl CompetitionResult {
        pub fn create_test_instance() -> Self {
            CompetitionResult::new(
                Competition::new(Convention::new(String::from("Convention")), String::from("100m")),
                Place::from_string("1").unwrap(),
                ResultType::from_string("Overall").unwrap(),
                String::from("00:14:99"),
                String::new(),
                String::from("Senior"),
            )
        }
    }
}