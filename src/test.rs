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

#[test]
fn tideman_example_5() {
    assert_eq!(
        tally(
            [
                std::iter::repeat_n([0, 1, 2, 3, 4].as_slice(), 7),
                std::iter::repeat_n([4, 3, 0, 1, 2].as_slice(), 3),
                std::iter::repeat_n([3, 4, 1, 2, 0].as_slice(), 6),
                std::iter::repeat_n([1, 2, 0, 4, 3].as_slice(), 3),
                std::iter::repeat_n([4, 2, 0, 1, 3].as_slice(), 5),
                std::iter::repeat_n([3, 2, 0, 1, 4].as_slice(), 3),
            ]
            .into_iter()
            .flatten()
            .collect::<Vec<_>>()
            .as_slice(),
            5
        )
        .unwrap(),
        BTreeSet::from([0]),
    );
}

#[test]
fn tideman_example_6() {
    assert_eq!(
        tally(&[[0, 1, 2, 3], [1, 2, 3, 0], [3, 2, 0, 1]], 4).unwrap(),
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
