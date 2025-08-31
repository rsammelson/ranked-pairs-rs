use std::collections::BTreeSet;

use crate::Error;

use super::tally;

#[test]
fn invalid_ballots() {
    assert_eq!(tally(&[[1, 2], [0, 3]], 3), Err(Error::InvalidCandidate));
    assert_eq!(tally(&[[0, 1, 2], [0, 1, 0]], 3), Err(Error::InvalidBallot),);
}

#[test]
fn basic() {
    for l in 0..3 {
        assert_eq!(
            tally(&(0..l).map(|_| []).collect::<Vec<_>>(), 0).unwrap(),
            BTreeSet::from([])
        );
    }

    for l in 0..3 {
        assert_eq!(
            tally(&(0..l).map(|_| []).collect::<Vec<_>>(), 1).unwrap(),
            BTreeSet::from([0])
        );
    }

    for l in 0..3 {
        assert_eq!(
            tally(&(0..l).map(|_| [0]).collect::<Vec<_>>(), 1).unwrap(),
            BTreeSet::from([0])
        );
    }
}

#[test]
fn simple() {
    assert_eq!(
        tally(
            &[[1, 2].as_slice(), [0, 3].as_slice(), [3, 2, 1].as_slice()],
            6
        )
        .unwrap(),
        BTreeSet::from([3]),
    );
    assert_eq!(
        tally(
            &[[1, 2].as_slice(), [0, 3].as_slice(), [3, 2, 1].as_slice()],
            6
        )
        .unwrap(),
        BTreeSet::from([3]),
    );
}

#[test]
fn wikipedia_example() {
    assert_eq!(
        tally(
            [
                std::iter::repeat_n([0, 1, 2, 3].as_slice(), 42),
                std::iter::repeat_n([1, 2, 3, 0].as_slice(), 26),
                std::iter::repeat_n([2, 3, 1, 0].as_slice(), 15),
                std::iter::repeat_n([3, 2, 1, 0].as_slice(), 17),
            ]
            .into_iter()
            .flatten()
            .collect::<Vec<_>>()
            .as_slice(),
            4
        )
        .unwrap(),
        BTreeSet::from([1]),
    );
}

#[test]
fn simple_tie() {
    assert_eq!(
        tally(
            [
                std::iter::repeat_n([0, 2].as_slice(), 8),
                std::iter::repeat_n([2, 3, 0, 1].as_slice(), 4),
                std::iter::repeat_n([2, 3, 1].as_slice(), 2),
                std::iter::repeat_n([3, 2].as_slice(), 2),
            ]
            .into_iter()
            .flatten()
            .collect::<Vec<_>>()
            .as_slice(),
            4
        )
        .unwrap(),
        BTreeSet::from([0, 2]),
    );
}

// Tideman examples from: Tideman, T.N. Independence of clones as a criterion for voting rules. Soc
// Choice Welfare 4, 185–206 (1987). https://doi.org/10.1007/BF00433944

/// Filter the ballots, keeping only candidates for which the function gives `true`
fn filter_ballots<B: Into<Vec<u16>>>(ballots: Vec<B>, f: impl Fn(u16) -> bool) -> Vec<Vec<u16>> {
    ballots
        .into_iter()
        .map(|b| b.into().into_iter().filter(|n| f(*n)).collect())
        .collect()
}

pub fn tideman_example_2_ballots() -> Vec<[u16; 5]> {
    [
        std::iter::repeat_n([0, 1, 2, 3, 4], 9),
        std::iter::repeat_n([1, 0, 2, 4, 3], 8),
        std::iter::repeat_n([2, 4, 3, 1, 0], 15),
        std::iter::repeat_n([3, 4, 0, 1, 2], 16),
    ]
    .into_iter()
    .flatten()
    .collect()
}

#[test]
fn tideman_example_2() {
    assert_eq!(
        tally(&tideman_example_2_ballots(), 5).unwrap(),
        // 0 & 1 beat 2, 2 beats 3 & 4, (ignored: 3 & 4 beat 0 & 1), 3 beats 4, 0 beats 1
        BTreeSet::from([0]),
    );

    // this has 4 (clone of 3) removed
    assert_eq!(
        tally(&filter_ballots(tideman_example_2_ballots(), |c| c != 4), 4).unwrap(),
        // should be the same as the first part due to independence of clones
        BTreeSet::from([0]),
    );

    // this has 1 (clone of 0) removed
    assert_eq!(
        tally(&filter_ballots(tideman_example_2_ballots(), |c| c != 1), 5).unwrap(),
        BTreeSet::from([0]),
    );

    // this has both clones removed
    assert_eq!(
        tally(
            &filter_ballots(tideman_example_2_ballots(), |c| c != 1 && c != 4),
            5
        )
        .unwrap(),
        BTreeSet::from([0]),
    );
}

pub fn tideman_example_3_ballots() -> Vec<[u16; 3]> {
    [
        std::iter::repeat_n([0, 1, 2], 3),
        std::iter::repeat_n([2, 1, 0], 2),
        std::iter::repeat_n([2, 0, 1], 2),
    ]
    .into_iter()
    .flatten()
    .collect()
}

#[test]
fn tideman_example_3() {
    assert_eq!(
        tally(&tideman_example_3_ballots(), 3).unwrap(),
        BTreeSet::from([2]),
    );

    // clone removed
    assert_eq!(
        tally(&filter_ballots(tideman_example_3_ballots(), |c| c != 0), 3).unwrap(),
        BTreeSet::from([2]),
    );
}

pub fn tideman_example_4_ballots() -> Vec<[u16; 4]> {
    [
        std::iter::repeat_n([0, 1, 2, 3], 6),
        std::iter::repeat_n([1, 2, 0, 3], 5),
        std::iter::repeat_n([2, 0, 1, 3], 4),
        std::iter::repeat_n([3, 0, 1, 2], 5),
        std::iter::repeat_n([3, 1, 2, 0], 4),
        std::iter::repeat_n([3, 2, 0, 1], 3),
    ]
    .into_iter()
    .flatten()
    .collect()
}

#[test]
fn tideman_example_4() {
    assert_eq!(
        tally(&tideman_example_4_ballots(), 4).unwrap(),
        // 1 beats 2, 0 beats 1, (ignored: 2 beats 0), 0 & 1 & 2 beat 3
        BTreeSet::from([0]),
    );
}

pub fn tideman_example_5_ballots() -> Vec<[u16; 5]> {
    [
        std::iter::repeat_n([0, 1, 2, 3, 4], 7),
        std::iter::repeat_n([4, 3, 0, 1, 2], 3),
        std::iter::repeat_n([3, 4, 1, 2, 0], 6),
        std::iter::repeat_n([1, 2, 0, 4, 3], 3),
        std::iter::repeat_n([4, 2, 0, 1, 3], 5),
        std::iter::repeat_n([3, 2, 0, 1, 4], 3),
    ]
    .into_iter()
    .flatten()
    .collect()
}

#[test]
fn tideman_example_5() {
    assert_eq!(
        tally(&tideman_example_5_ballots(), 5).unwrap(),
        BTreeSet::from([0]),
    );

    // remove a clone
    assert_eq!(
        tally(&filter_ballots(tideman_example_5_ballots(), |c| c != 2), 5).unwrap(),
        BTreeSet::from([0]),
    );

    // remove the other clone
    assert_eq!(
        tally(&filter_ballots(tideman_example_5_ballots(), |c| c != 1), 5).unwrap(),
        BTreeSet::from([2]), // changes to a different candidate in the set of clones, this is okay
    );

    // remove both clones
    assert_eq!(
        tally(
            &filter_ballots(tideman_example_5_ballots(), |c| c != 1 && c != 2),
            5
        )
        .unwrap(),
        BTreeSet::from([0]),
    );
}

pub fn tideman_example_6_ballots() -> Vec<[u16; 4]> {
    vec![[0, 1, 2, 3], [1, 2, 3, 0], [3, 2, 0, 1]]
}

#[test]
fn tideman_example_6() {
    assert_eq!(
        tally(&tideman_example_6_ballots(), 4).unwrap(),
        BTreeSet::from([0, 1, 2, 3]),
    );
}

// Munger examples from: Munger, C.T. The best Condorcet-compatible election method: Ranked Pairs.
// Const Polit Econ 34, 434–444 (2023). https://doi.org/10.1007/s10602-022-09382-w

#[test]
fn munger_example_1() {
    assert_eq!(
        tally(
            [
                std::iter::repeat_n([0, 2, 3, 1].as_slice(), 3),
                std::iter::repeat_n([0, 3, 1, 2].as_slice(), 5),
                std::iter::repeat_n([1, 0, 2, 3].as_slice(), 4),
                std::iter::repeat_n([1, 2, 3, 0].as_slice(), 5),
                std::iter::repeat_n([2, 0, 3, 1].as_slice(), 2),
                std::iter::repeat_n([2, 3, 0, 1].as_slice(), 5),
                std::iter::repeat_n([3, 0, 1, 2].as_slice(), 2),
                std::iter::repeat_n([3, 1, 0, 2].as_slice(), 4),
            ]
            .into_iter()
            .flatten()
            .collect::<Vec<_>>()
            .as_slice(),
            4
        )
        .unwrap(),
        BTreeSet::from([3]),
    );
}
