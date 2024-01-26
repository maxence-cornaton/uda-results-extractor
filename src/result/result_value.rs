use std::cmp::Ordering;
use std::cmp::Ordering::{Equal, Greater, Less};
use std::collections::HashMap;
use std::time::Duration;

use regex::{Match, Regex};

use crate::competition::competition::Competition;
use crate::competition::competition_result::CompetitionResult;
use crate::result::result_value::ResultValue::{Distance, Empty, Points, Time};

thread_local!(static TIME_REGEX: Regex = Regex::new(r"^((\d):)?(\d\d):(\d\d)(\.(\d\d\d?))?$").unwrap());
thread_local!(static POINTS_REGEX: Regex = Regex::new(r"^(\d+\.\d+) pts").unwrap());
thread_local!(static DISTANCE_REGEX: Regex = Regex::new(r"^(\d+) cm").unwrap());

/// Results may be of different types:
/// - Empty
/// - Time
/// - Distance (cm)
/// - Points
/// - Custom
#[derive(Debug, Clone, PartialEq)]
pub enum ResultValue {
    Empty,
    Time(Duration),
    Points(f32),
    Distance(u16),
    Custom(String),
}

impl ResultValue {
    pub fn from_string(result: &str) -> Self {
        if result.is_empty() {
            return Empty;
        }
        let duration = try_parse_duration(result);
        if duration.is_some() {
            return Time(duration.unwrap());
        }
        let points = try_parse_points(result);
        if points.is_some() {
            return Points(points.unwrap());
        }
        let distance = try_parse_distance(result);
        if distance.is_some() {
            return Distance(distance.unwrap());
        }
        ResultValue::Custom(String::from(result))
    }

    pub fn compare(&self, other: &Self) -> Result<Ordering, String> {
        match self {
            Empty => { Err(format!("Can't compare empty value [self: {:?},other: {:?}]", self, other)) }
            Time(self_time) => {
                match other {
                    Empty => { Err(format!("Can't compare time with empty value [self: {:?},other: {:?}]", self, other)) }
                    Time(other_time) => { Ok(self_time.cmp(other_time)) }
                    Points(_) => { Err(format!("Can't compare time with points [self: {:?},other: {:?}]", self, other)) }
                    Distance(_) => { Err(format!("Can't compare time with distance [self: {:?},other: {:?}]", self, other)) }
                    ResultValue::Custom(_) => { Err(format!("Can't compare time with custom value [self: {:?},other: {:?}]", self, other)) }
                }
            }
            Points(self_points) => {
                match other {
                    Empty => { Err(format!("Can't compare points with empty value [self: {:?},other: {:?}]", self, other)) }
                    Time(_) => { Err(format!("Can't compare points with time [self: {:?},other: {:?}]", self, other)) }
                    Points(other_points) => { Ok(self_points.partial_cmp(other_points).unwrap()) }
                    Distance(_) => { Err(format!("Can't compare points with distance [self: {:?},other: {:?}]", self, other)) }
                    ResultValue::Custom(_) => { Err(format!("Can't compare points with custom value [self: {:?},other: {:?}]", self, other)) }
                }
            }
            Distance(self_distance) => {
                match other {
                    Empty => { Err(format!("Can't compare distance with empty value [self: {:?},other: {:?}]", self, other)) }
                    Time(_) => { Err(format!("Can't compare distance with time [self: {:?},other: {:?}]", self, other)) }
                    Points(_) => { Err(format!("Can't compare distance with points [self: {:?},other: {:?}]", self, other)) }
                    Distance(other_distance) => { Ok(self_distance.cmp(other_distance)) }
                    ResultValue::Custom(_) => { Err(format!("Can't compare distance with custom value [self: {:?},other: {:?}]", self, other)) }
                }
            }
            ResultValue::Custom(_) => { Err(format!("Can't compare custom value [self: {:?},other: {:?}]", self, other)) }
        }
    }
}

// region Duration
/// Try and parse duration in the following formats:
/// - mm:ss.zz
/// - mm:ss.zzz
/// - h:mm:ss.zzz
fn try_parse_duration(result: &str) -> Option<Duration> {
    let Some(fields) = TIME_REGEX.with(|regex| regex.captures(result)) else { return None; };

    let hours = get_duration_field_value(fields.get(2));
    let minutes = get_duration_field_value(fields.get(3));
    let seconds = get_duration_field_value(fields.get(4));
    let thousands = get_duration_field_value(fields.get(6));

    Some(Duration::new((hours * 3600 + minutes * 60 + seconds) as u64, thousands * 1_000_000))
}

fn get_duration_field_value(field: Option<Match>) -> u32 {
    match field {
        None => 0,
        Some(m) => m.as_str().parse::<u32>().unwrap()
    }
}
// endregion

// region Points
/// Try and parse points in the following formats: "\d+\.\d+ pts"
fn try_parse_points(result: &str) -> Option<f32> {
    let Some(fields) = POINTS_REGEX.with(|regex| regex.captures(result)) else { return None; };

    let points = match fields.get(1) {
        None => None,
        Some(m) => Some(m.as_str().parse::<f32>().unwrap())
    };

    points
}
// endregion

// region Distance
/// Try and parse distance in the following formats: "\d+ cm"
fn try_parse_distance(result: &str) -> Option<u16> {
    let Some(fields) = DISTANCE_REGEX.with(|regex| regex.captures(result)) else { return None; };

    let distance = match fields.get(1) {
        None => None,
        Some(m) => Some(m.as_str().parse::<u16>().unwrap())
    };

    distance
}
// endregion

pub fn get_best_results<'a>(results: &'a Vec<CompetitionResult>, higher_is_better_for_competition: &'a HashMap<Competition, bool>)
                            -> HashMap<&'a Competition, &'a CompetitionResult> {
    let mut best_values = HashMap::new();

    for result in results {
        if result.result().is_none() {
            continue;
        }

        let competition = result.competition();
        match best_values.get(competition) {
            None => { best_values.insert(competition, result); }
            Some(best_result_so_far) => {
                let ordering = result.result().as_ref().unwrap().compare(best_result_so_far.result().as_ref().unwrap());
                // FIXME: add debug information
                if ordering.is_ok() {
                    let ordering = ordering.unwrap();
                    let higher_is_better = higher_is_better_for_competition.get(competition);
                    if higher_is_better.is_none() {
                        eprintln!("No information on what's better for competition [competition: {:?}]", competition);
                        continue;
                    }
                    let higher_is_better = *higher_is_better.unwrap();
                    if higher_is_better {
                        if ordering == Greater {
                            best_values.insert(competition, result);
                        } else if ordering == Equal {
                            // FIXME: should be added
                        } else if ordering == Less {
                            // FIXME: add trace
                        }
                    } else {
                        if ordering == Less {
                            best_values.insert(competition, result);
                        } else if ordering == Equal {
                            // FIXME: should be added
                        } else if ordering == Greater {
                            // FIXME: add trace
                        }
                    }
                } else {
                    eprintln!("Can't compare results [best_result_so_far: {:?}, result: {:?}]", best_result_so_far, result);
                    eprintln!("{}", ordering.err().unwrap());
                }
            }
        };
    }
    best_values
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::competition::competition::Competition;
    use crate::competition::competition_result::CompetitionResult;
    use crate::result::age_group::AgeGroup;
    use crate::result::place::Place;
    use crate::result::result_type::ResultType;
    use crate::result::result_value::{get_best_results, ResultValue};

    #[test]
    fn should_get_best_result() {
        let result1 = CompetitionResult::new(
            1,
            Competition::new("Competition"),
            Place::from_string("1").unwrap(),
            ResultType::from_string("Overall").unwrap(),
            Some(ResultValue::from_string("00:14.99")),
            None,
            Some(AgeGroup::from_string("Senior")),
        );
        let result2 = CompetitionResult::new(
            2,
            Competition::new("Competition"),
            Place::from_string("2").unwrap(),
            ResultType::from_string("Overall").unwrap(),
            Some(ResultValue::from_string("00:18.00")),
            None,
            Some(AgeGroup::from_string("Senior")),
        );
        let results = vec![result1, result2.clone()];

        let mut higher_is_better_for_competition = HashMap::new();
        higher_is_better_for_competition.insert(Competition::new("Competition"), true);

        let best_results = get_best_results(&results, &higher_is_better_for_competition);
        println!("{:?}", best_results);

        let best_result = *best_results.get(&Competition::new("Competition")).unwrap();
        assert_eq!(best_result, &result2);
    }
}