pub mod reaction;

use std::{collections::{HashSet, HashMap}};
use reaction::{Reaction, term::species::Species};

/// Contains a set of all the declared reactions, as well as a set of all of the available species.
struct ReactionNetwork <'reacting> {
    reactions: HashSet<Reaction<'reacting>>,
    solution: HashMap<&'reacting Species, Species>,
}

impl<'reacting> ReactionNetwork <'reacting> {

    /// Randomly selects a reaction from the provided set using a probablility generated from the total number of reactions with reach one scaled by reaction rate,
    pub fn get_next_reaction(possible_reactions: HashSet<Reaction<'reacting>>) -> &'reacting Reaction<'reacting> {
        
        index = rand::random()
        
        return next_reaction;
    }

    /// Updates local solution with the effects of a given reaction
    pub fn react(& mut self, reaction: &'reacting Reaction<'reacting>) {
        
        // this could maybe be done by using dedicated functions in Term struct which would allow for storage of species in a hash set
        for reactant in reaction.get_reactants() {
            self.solution.entry(reactant.get_species())
            .and_modify(|species| species.set_count(species.get_count() - reactant.get_coefficient()));
        }

        for product in reaction.get_products() {
            self.solution.entry(product.get_species())
            .and_modify(|species| species.set_count(species.get_count() + product.get_coefficient()));
        }

        return;
    }

    /// Returns the subset of local reactions set is possible based on the Species.count values in solution
    fn get_possible_reactions (&self) -> HashSet<Reaction<'reacting>> {
    
        let mut possible_reactions: HashSet<Reaction<'reacting>> = HashSet::new();

            for reaction in &self.reactions {
                if reaction.is_possible() {
                    if possible_reactions.insert(reaction.clone()) { }
                }
            }

        return possible_reactions;
    }

    /// takes outputs the total sum of the reaction rates in the provided hash set of reactions
    fn sum_reaction_rates (reactions: HashSet<Reaction<'reacting>>) -> u64 {
        let mut sum: u64 = 0; 
        for reaction in reactions {
            sum += reaction.get_reaction_rate();
        }
        return sum;
    }


}
