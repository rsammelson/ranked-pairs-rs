#![warn(missing_docs)]
#![feature(btree_cursors)]

//! Ranked pairs (Tideman method) election method, with handling of ties.

mod graph;
mod pairwise;

#[cfg(test)]
mod test;

use std::collections::{BTreeSet, HashSet};

use itertools::Itertools as _;

/// Tally election results
///
/// Each ballot consists of a list of choices in order, candidate numbers are zero-based. The
/// function returns the set of winners, using the algorithm in "Independence of clones as a
/// criterion for voting rules" (Tideman, 1986). Specifically, as each winning margin is added to
/// the graph, every possible order is considered. Any candidate who is can win in any scenario is
/// considered to be in the winning set.
///
/// # Errors
/// An error will be returned if any ballot contains an invalid candidate number (`>= candidates`)
/// or contains the same candidate more than once.
pub fn tally<B: AsRef<[u16]>>(ballots: &[B], candidates: u16) -> Result<BTreeSet<u16>, Error> {
    check_ballots(ballots, candidates)?;

    // return easy cases immediately
    match candidates {
        0 => return Ok(BTreeSet::from([])),
        1 => return Ok(BTreeSet::from([0])),
        _ => {}
    }

    let pairwise_results = pairwise::tabulate_pairwise_results(ballots, candidates);

    // create a graph
    let mut graphs = HashSet::from([graph::AcyclicGraph::new(candidates)]);

    println!("{pairwise_results:?}");

    // iterate over each group of equal-margin pairings, in order from largest margin of victory to
    // smallest (reverse of usual order)
    for pairings in pairwise_results.into_values().rev() {
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

    // return nodes with no incoming edges (unbeaten candidates) in any possible scenario
    Ok(graphs.iter().flat_map(|graph| graph.roots()).collect())
}

fn check_ballots<B: AsRef<[u16]>>(ballots: &[B], candidates: u16) -> Result<(), Error> {
    for ballot in ballots {
        let ballot = ballot.as_ref();

        // tracker for what candidates have been seen in this ballot
        let mut selections = range_set_blaze::RangeSetBlaze::new();

        for v in ballot {
            if *v >= candidates {
                // invalid candidate number
                return Err(Error::InvalidCandidate);
            }

            let v = usize::from(*v);
            if !selections.insert(v) {
                // duplicate candidate number
                return Err(Error::InvalidBallot);
            }
        }
    }

    Ok(())
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
