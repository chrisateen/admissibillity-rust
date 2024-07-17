use graphbench::graph::{Vertex, VertexMap, VertexSet};

pub struct MatchingEdges {
    //key vertex v in L, value neighbour of v in M and in R
    pub e_add: VertexMap<Vertex>,
    pub e_remove: VertexMap<Vertex>,
}

pub struct AugmentingPath {
    pub id: Vertex,
    pub s: VertexSet,
    pub t: VertexSet,
    pub out: VertexMap<Vertex>,
    pub edges: VertexMap<VertexSet>,
}

impl AugmentingPath {
    pub fn new(v: Vertex) -> Self {
        AugmentingPath {
            id: v,
            s: VertexSet::default(),
            t: VertexSet::default(),
            out: VertexMap::default(),
            edges: VertexMap::default(),
        }
    }

    pub fn should_do_augmenting_path(&self) -> bool {
        !self.s.is_empty() && !self.t.is_empty()
    }

    fn dfs(&self, v: Vertex) -> Option<Vec<Vertex>> {
        let mut path = vec![v];
        let mut visited = VertexSet::default();
        visited.insert(v);

        'main: while !path.is_empty() {
            let u = path.last().unwrap();
            if let Some(u_neighbours) = self.edges.get(u) {
                for w in u_neighbours {
                    if !visited.contains(w) {
                        path.push(*w);
                        visited.insert(*w);
                        if self.t.contains(w) {
                            return Some(path);
                        }
                        continue 'main;
                    }
                }
            };
            path.pop();
        }
        None
    }

    fn get_new_matching_edges(&self, path: Vec<Vertex>) -> MatchingEdges {
        let mut edges = MatchingEdges {
            e_add: VertexMap::default(),
            e_remove: VertexMap::default(),
        };

        for c in path.chunks(2) {
            edges.e_remove.insert(c[1], c[0]);
        }

        let first = path[0];
        edges.e_add.insert(*self.out.get(&first).unwrap(), first);

        let last = path[path.len() - 1];
        edges.e_add.insert(last, *self.out.get(&last).unwrap());

        for c in path[1..path.len() - 1].chunks(2) {
            edges.e_add.insert(c[0], c[1]);
        }

        edges
    }

    pub fn find_augmenting_path(&self) -> Option<MatchingEdges> {
        //If there is no start an end points there is no need doing augmenting path
        if !self.should_do_augmenting_path() {
            return None;
        }

        for v in &self.s {
            match self.dfs(*v) {
                None => {}
                Some(path) => {
                    let m = self.get_new_matching_edges(path);
                    return Some(m);
                }
            }
        }
        None
    }
}

#[cfg(test)]
mod test_augmenting_path {
    use crate::augmentingPath::AugmentingPath;
    use graphbench::graph::Vertex;

    #[test]
    fn test_should_do_augmenting_path_should_return_false_if_s_and_t_is_empty() {
        let aug_path = AugmentingPath::new(1);
        assert!(!aug_path.should_do_augmenting_path());
    }

    #[test]
    fn test_should_do_augmenting_path_should_return_false_if_s_is_empty() {
        let mut aug_path = AugmentingPath::new(1);
        aug_path.t.insert(2);
        assert!(!aug_path.should_do_augmenting_path());
    }

    #[test]
    fn test_should_do_augmenting_path_should_return_false_if_t_is_empty() {
        let mut aug_path = AugmentingPath::new(1);
        aug_path.s.insert(2);
        assert!(!aug_path.should_do_augmenting_path());
    }

    #[test]
    fn test_should_do_augmenting_path_should_return_true_if_s_and_t_is_not_empty() {
        let mut aug_path = AugmentingPath::new(1);
        aug_path.s.insert(2);
        aug_path.t.insert(3);
        assert!(aug_path.should_do_augmenting_path());
    }

    #[test]
    fn test_get_new_matching_edges_should_return_edges_in_matching_and_edges_to_remove_from_matching(
    ) {
        let mut aug_path = AugmentingPath::new(1);
        aug_path.s.extend([2, 6]);
        aug_path.t.extend([7, 9]);
        let mut path: Vec<Vertex> = Vec::new();
        [2, 3, 4, 5, 6, 7, 8, 9].map(|x| path.push(x));
        aug_path.out.insert(2, 10);
        aug_path.out.insert(6, 11);
        aug_path.out.insert(7, 12);
        aug_path.out.insert(9, 13);

        let edges = aug_path.get_new_matching_edges(path);

        assert!(edges.e_add.contains_key(&10));
        assert!(edges.e_remove.contains_key(&3));
    }

    #[test]
    fn test_find_augmenting_path_should_return_edges_in_matching() {
        let mut aug_path = AugmentingPath::new(1);
        aug_path.s.extend([2, 6]);
        aug_path.t.extend([7, 9]);
        aug_path.out.insert(2, 10);
        aug_path.out.insert(6, 11);
        aug_path.out.insert(7, 12);
        aug_path.out.insert(9, 13);

        let edges = [(2, 3), (3, 4), (4, 5), (5, 6), (6, 7), (7, 8), (8, 9)];
        for (v, u) in edges {
            aug_path.edges.entry(v).or_default().insert(u);
        }

        let path = aug_path.find_augmenting_path();

        assert!(path.is_some());
    }
}
