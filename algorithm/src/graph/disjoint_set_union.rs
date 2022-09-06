pub struct DSUNode {
    parent: usize,
    size: usize,
}

pub struct DisjointSetUnion {
    // vector index is set NO.
    nodes: Vec<DSUNode>,
}

impl DisjointSetUnion {
    pub fn new(n: usize) -> DisjointSetUnion {
        let mut nodes = Vec::new();
        nodes.reserve_exact(n);
        for i in 0..n {
            nodes.push(DSUNode { parent: i, size: 1 });
        }
        DisjointSetUnion { nodes }
    }

    pub fn find_set(& self, v: usize) -> usize {
        let mut p = v;
        while p != self.nodes[p].parent {
            p = self.nodes[p].parent
        }

        p
    }

    pub fn merge(&mut self, u: usize, v: usize) -> usize {
        let mut a = self.find_set(u);
        let mut b = self.find_set(v);
        if a == b {
            return std::usize::MAX;
        }
        if self.nodes[a].size < self.nodes[b].size {
            std::mem::swap(&mut a, &mut b);
        }
        self.nodes[b].parent = a;
        self.nodes[a].size += self.nodes[b].size;

        a
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_acyclic_graph() {
        let mut dsu = DisjointSetUnion::new(10);
        let edges: Vec<(usize, usize)> = vec![
            (1, 2),
            (2, 1),
            (2, 3),
            (1, 3),
            (4, 5),
            (7, 8),
            (4, 8),
            (3, 8),
            (1, 9),
            (2, 9),
            (3, 9),
            (4, 9),
            (5, 9),
            (6, 9),
            (7, 9),
        ];
        let expected_edges: Vec<(usize, usize)> = vec![
            (1, 2),
            (2, 3),
            (4, 5),
            (7, 8),
            (4, 8),
            (3, 8),
            (1, 9),
            (6, 9),
        ];
        let mut added_edges: Vec<(usize, usize)> = Vec::new();
        for (u, v) in edges {
            if dsu.merge(u, v) < std::usize::MAX {
                added_edges.push((u, v));
            }
            assert!(dsu.merge(u, v) == std::usize::MAX);
        }
        assert_eq!(added_edges, expected_edges);
        let comp_1 = dsu.find_set(1);
        for i in 2..=9 {
            assert_eq!(comp_1, dsu.find_set(i));
        }
        assert_ne!(comp_1, dsu.find_set(0));
    }
}
