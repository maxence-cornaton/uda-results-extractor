use derive_getters::Getters;

use crate::competitor::competitor::Competitor;

#[derive(Debug, Getters, PartialEq, Eq, Hash, Clone)]
pub struct Team {
    name: String,
    members: Vec<Competitor>,
}