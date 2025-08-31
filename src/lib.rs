#![warn(missing_docs)]
#![feature(btree_cursors)]

//! Ranked pairs (Tideman method) election method, with handling of ties.

mod graph;
mod pairwise;

#[cfg(test)]
mod test;

use std::collections::{BTreeMap, BTreeSet, HashSet};

use itertools::Itertools as _;

/// Tabulated data for an election
///
/// This contains the minimal data needed from the ballots to be able to compute the winners,
/// without storing all of the ballots.
#[derive(Debug)]
pub struct TabulatedData {
    table: BTreeMap<usize, BTreeSet<(u16, u16)>>,
    candidates: u16,
}

impl TabulatedData {
    /// Create the data from a set of ballots
    ///
    /// Each ballot consists of a list of choices in order, candidate numbers are zero-based.
    ///
    /// # Errors
    /// An error will be returned if any ballot contains an invalid candidate number (`>= candidates`)
    /// or contains the same candidate more than once.
    pub fn from_ballots<B: AsRef<[u16]>>(
        ballots: impl IntoIterator<Item = B> + Copy,
        candidates: u16,
    ) -> Result<Self, Error> {
        Ok(Self {
            table: pairwise::tabulate_pairwise_results(ballots, candidates)?,
            candidates,
        })
    }

    /// Tally election results
    ///
    /// The function returns the set of winners, using the algorithm in "Independence of clones as
    /// a criterion for voting rules" (Tideman, 1986). Specifically, as each winning margin is
    /// added to the graph, every possible order is considered. Any candidate who is can win in any
    /// scenario is considered to be in the winning set.
    pub fn tally(&self) -> BTreeSet<u16> {
        // create a graph
        let mut graphs = HashSet::from([graph::AcyclicGraph::new(self.candidates)]);

        // iterate over each group of equal-margin pairings, in order from largest margin of
        // victory to smallest (reverse of usual order)
        for pairings in self.pairwise_results() {
            // get every possible ordering of the pairings with this margin of victory
            let possible_match_orders = pairings.iter().copied().permutations(pairings.len());

            // take every possible graph so far and modify it in each possible order
            graphs = graphs
                .into_iter()
                .cartesian_product(possible_match_orders)
                .map(|(mut graph, matches)| {
                    for (winner, loser) in matches {
                        // just skip adding if it would add a cycle
                        graph.try_add_edge(winner, loser);
                    }
                    graph
                })
                .collect();
        }

        graphs.iter().flat_map(|graph| graph.roots()).collect()
    }

    /// Get each set of non-tied pairwise elections
    ///
    /// The sets are of elections with the same margin of victory. The values are in order from
    /// widest margin of victory to slimmest.
    pub fn pairwise_results(&self) -> impl Iterator<Item = &BTreeSet<(u16, u16)>> {
        self.table.values().rev()
    }
}

/// Tally election results
///
/// This is a shortcut for [TabulatedData::from_ballots] followed by [TabulatedData::tally].
pub fn tally<B: AsRef<[u16]>>(ballots: &[B], candidates: u16) -> Result<BTreeSet<u16>, Error> {
    TabulatedData::from_ballots(ballots, candidates).map(|d| d.tally())
}

/// An error while tallying an election
#[derive(thiserror::Error, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum Error {
    /// A ballot had a duplicate choice
    #[error("an invalid ballot was given")]
    InvalidBallot,
    /// A ballot contained an invalid candidate number
    #[error("an invalid candidate was voted for")]
    InvalidCandidate,
}
