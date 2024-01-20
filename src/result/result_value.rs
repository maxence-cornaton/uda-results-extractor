use std::time::Duration;

use regex::{Match, Regex};

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