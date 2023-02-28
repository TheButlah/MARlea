use std::{collections::{HashSet, HashMap}};
#[derive(Hash, Eq, PartialEq)]

/// Contains a Name and value representing the count of some named DNA string
struct Species {
    name: String,
    count: u64,
}

/// Contains the data for a single term within a larger reaction.
/// Species is a reference to a named value in solution which will be added to or subtracted from. 
/// Coefficient is the value to add or subtract
struct  Term <'reaction> {
    species: &'reaction Species,
    coefficient: u8,
}

/// Stores a set for the reaction reactants and products. 
/// Eeach element contains the variable key used by a Solution struct as well as a reaction rate. 
/// This struct should only be used inside of the Reaction_Network Struct 
struct Reaction <'reaction> {
    reactants: HashSet<Term<'reaction,>>,
    products: HashSet<Term<'reaction,>>,
    reaction_rate: u32,
}   

/// Contains a set of all the declared reactions, as well as a set of all of the avilable species.
struct ReactionNework <'reacting> {
    reactions: HashSet<Reaction<'reacting>>,
    solution: HashMap<&'reacting Species, Species>,
}

impl<'reacting> ReactionNework <'reacting> {

    /// Randomly selects a reaction from possible_next_reactions based on the probability in reaction rate,
    fn get_next_reaction(&self) -> &Reaction {
        let mut next_reaction: &Reaction = todo!();
    
        
        return next_reaction;
    }

    /// simulates the effects of the given reaction occuring in the reaction network
    fn react(& mut self, reaction: &'reacting Reaction<'reacting>) {
        
        for reactant in &reaction.reactants {
            self.solution.entry(reactant.species)
            .and_modify(|species| species.count -= u64::from(reactant.coefficient));
        }

        for product in &reaction.products {
            self.solution.entry(product.species)
            .and_modify(|species| species.count -= u64::from(product.coefficient));
        }
    }
}

fn main () {

}