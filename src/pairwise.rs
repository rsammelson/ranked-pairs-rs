use std::collections::{BTreeMap, BTreeSet};

pub fn tabulate_pairwise_results<B: AsRef<[u16]>>(
    ballots: &[B],
    candidates: u16,
) -> BTreeMap<usize, BTreeSet<(u16, u16)>> {
    let mut pairwise_results: BTreeMap<usize, BTreeSet<(u16, u16)>> = BTreeMap::new();

    // iterate over each unique pairing
    for c1 in 0..candidates - 1 {
        for c2 in c1 + 1..candidates {
            let (c1_wins, c2_wins) = count_pairwise_election(ballots, c1, c2);
            match c1_wins.cmp(&c2_wins) {
                // c1 won less than c2, so add c2 beating c1 by the margin
                std::cmp::Ordering::Less => assert!(
                    pairwise_results
                        .entry(c2_wins - c1_wins)
                        .or_default()
                        .insert((c2, c1))
                ),
                std::cmp::Ordering::Equal => {
                    // ties don't matter, so ignore
                }
                // c1 won more than c2, so add c1 beating c2 by the margin
                std::cmp::Ordering::Greater => assert!(
                    pairwise_results
                        .entry(c1_wins - c2_wins)
                        .or_default()
                        .insert((c1, c2))
                ),
            }
        }
    }

    pairwise_results
}

fn count_pairwise_election<B: AsRef<[u16]>>(ballots: &[B], c1: u16, c2: u16) -> (usize, usize) {
    let mut c1_wins = 0;
    let mut c2_wins = 0;
    for ballot in ballots {
        match ballot
            .as_ref()
            .iter()
            .copied()
            .find(|e| *e == c1 || *e == c2)
        {
            // c1 was ranked before c2
            Some(v) if v == c1 => c1_wins += 1,
            // c2 was ranked before c1
            Some(v) if v == c2 => c2_wins += 1,
            // this shouldn't happen, since only values that are either c1 or c2 are found
            Some(_) => unreachable!(),
            // neither candidate was ranked on this ballot
            None => {}
        }
    }
    (c1_wins, c2_wins)
}

#[cfg(test)]
mod test {
    use std::collections::{BTreeMap, BTreeSet};

    use super::{count_pairwise_election, tabulate_pairwise_results};

    const BALLOTS: &[&[u16]] = &[
        [0, 1, 2].as_slice(),
        [1, 0, 2].as_slice(),
        [1, 2, 0].as_slice(),
        [1].as_slice(),
        [4].as_slice(),
    ];

    #[test]
    fn count() {
        assert_eq!(count_pairwise_election(BALLOTS, 0, 1), (1, 3));
        assert_eq!(count_pairwise_election(BALLOTS, 1, 0), (3, 1));
        assert_eq!(count_pairwise_election(BALLOTS, 0, 4), (3, 1));
        assert_eq!(count_pairwise_election(BALLOTS, 4, 1), (1, 4));
        assert_eq!(count_pairwise_election(BALLOTS, 2, 0), (1, 2));
        assert_eq!(count_pairwise_election(BALLOTS, 4, 5), (1, 0));
        assert_eq!(count_pairwise_election(BALLOTS, 8, 9), (0, 0));

        assert_eq!(count_pairwise_election(BALLOTS, 1, 2), (4, 0));
    }

    #[test]
    fn tabulate() {
        assert_eq!(
            tabulate_pairwise_results(BALLOTS, 6),
            BTreeMap::from([
                (4, BTreeSet::from([(1, 2), (1, 3), (1, 5)])),
                (3, BTreeSet::from([(0, 3), (0, 5), (1, 4), (2, 3), (2, 5)])),
                (2, BTreeSet::from([(1, 0), (0, 4), (2, 4)])),
                (1, BTreeSet::from([(0, 2), (4, 3), (4, 5)])),
            ])
        );
    }
}
