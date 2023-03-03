use species::{Species};

pub mod species;

/// Contains the data for a single term within a larger reaction.
/// Species is a reference to a named value in solution which will be added to or subtracted from. 
/// Coefficient is the value to add or subtract
#[derive(Eq, PartialEq,Clone)]
pub struct  Term<'reaction> {
    pub species: &'reaction Species,
    pub coefficient: u8,
}

impl<'reaction> std::hash::Hash for Term<'reaction> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.species.hash(state);
        self.coefficient.hash(state);
    }
}
