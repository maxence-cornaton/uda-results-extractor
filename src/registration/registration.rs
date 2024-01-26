use derive_getters::Getters;

use crate::convention::convention::Convention;

#[derive(Clone, Debug, Getters, PartialEq, Eq, Hash)]
pub struct Registration<'a> {
    convention: &'a Convention,
    id: u16,
}

impl<'a> Registration<'a> {
    pub fn new(convention: &'a Convention, id: u16) -> Self {
        Self { convention, id }
    }
}