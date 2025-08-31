use std::collections::BTreeSet;

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct AcyclicGraph {
    nodes: u16,
    edges: BTreeSet<(u16, u16)>,
}

impl AcyclicGraph {
    pub fn new(nodes: u16) -> Self {
        Self {
            nodes,
            edges: BTreeSet::new(),
        }
    }

    /// Try to add an edge, fails if it would cause a cycle
    ///
    /// Returns true if adding the edge succeeds and it is new.
    pub fn try_add_edge(&mut self, src: u16, dst: u16) -> bool {
        assert!(src < self.nodes);
        assert!(dst < self.nodes);

        if src != dst && self.dfs(dst).all(|e| e != src) {
            self.edges.insert((src, dst))
        } else {
            false
        }
    }

    pub fn roots(&self) -> impl Iterator<Item = u16> {
        #[allow(clippy::single_range_in_vec_init)]
        let mut elements = rangemap::RangeSet::new();
        if self.nodes > 0 {
            elements.insert(0..self.nodes);
            for e in &self.edges {
                elements.remove(e.1..e.1 + 1);
            }
        }
        elements.into_iter().flatten()
    }

    fn dfs(&self, start: u16) -> impl Iterator<Item = u16> {
        debug_assert!(start < self.nodes);

        Dfs {
            graph: self,
            visited: Vec::new(),
            node: start,
        }
    }

    fn outgoing(&self, src: u16) -> impl Iterator<Item = u16> {
        debug_assert!(src < self.nodes);

        self.edges
            .iter()
            .copied()
            .filter_map(move |(s, d)| (s == src).then_some(d))
    }
}

struct Dfs<'g> {
    graph: &'g AcyclicGraph,
    visited: Vec<(u16, u16)>,
    node: u16,
}

impl Iterator for Dfs<'_> {
    type Item = u16;
    fn next(&mut self) -> Option<Self::Item> {
        match self.graph.outgoing(self.node).next() {
            Some(dst) => {
                self.visited.push((self.node, dst));
                self.node = dst;
                Some(dst)
            }
            None => loop {
                match self.visited.pop() {
                    Some(last) => {
                        let mut cursor = self
                            .graph
                            .edges
                            .upper_bound(std::ops::Bound::Included(&last));
                        while let Some((src, dst)) = cursor.next().copied() {
                            if src == last.0 {
                                self.visited.push((src, dst));
                                self.node = dst;
                                return Some(dst);
                            }
                        }
                    }
                    None => return None,
                }
            },
        }
    }
}

impl std::iter::FusedIterator for Dfs<'_> {}

#[cfg(test)]
mod test {
    use crate::graph::AcyclicGraph;

    #[test]
    fn dfs() {
        let mut graph = AcyclicGraph::new(12);

        // root to each child
        assert!(graph.try_add_edge(8, 9));
        assert!(graph.try_add_edge(8, 2));
        assert!(graph.try_add_edge(8, 3));

        // left child of root
        assert!(graph.try_add_edge(9, 10));
        assert!(graph.try_add_edge(10, 11));
        assert!(graph.try_add_edge(10, 0));
        assert!(graph.try_add_edge(9, 1));

        // right child of root
        assert!(graph.try_add_edge(3, 4));
        assert!(graph.try_add_edge(4, 5));
        assert!(graph.try_add_edge(4, 6));
        assert!(graph.try_add_edge(3, 7));

        assert_eq!(
            graph.dfs(8).collect::<Vec<_>>(),
            &[2, 3, 4, 5, 6, 7, 9, 1, 10, 0, 11]
        );
    }
}
