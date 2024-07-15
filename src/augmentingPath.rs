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
    pub out: VertexMap<VertexSet>,
    pub edges: VertexMap<Vertex>,
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

    fn dfs(&self, v: Vertex, visited: &mut VertexMap<bool>, path: &mut Vec<Vertex>) {
        visited.insert(v, true);
        path.push(v);

        match self.edges.get(&v) {
            None => (),
            Some(v_neighbour) => match visited.get(v_neighbour) {
                None => {
                    self.dfs(*v_neighbour, visited, path);
                }
                Some(has_visited) => {
                    if !has_visited {
                        self.dfs(*v_neighbour, visited, path);
                    }
                }
            },
        }
    }

    fn get_new_matching_edges(&self, path: &mut Vec<Vertex>) -> MatchingEdges {
        let mut edges = MatchingEdges {
            e_add: VertexMap::default(),
            e_remove: VertexMap::default(),
        };

        for c in path.chunks(2) {
            edges
                .e_remove
                .insert(*c.get(1).unwrap(), *c.first().unwrap());
        }

        let first = path.remove(0);
        edges
            .e_add
            .insert(*self.out.get(&first).unwrap().iter().next().unwrap(), first);

        let last = path.remove(path.len() - 1);
        edges
            .e_add
            .insert(last, *self.out.get(&last).unwrap().iter().next().unwrap());

        for c in path.chunks(2) {
            edges.e_add.insert(*c.first().unwrap(), *c.get(1).unwrap());
        }

        edges
    }

    pub fn find_augmenting_path(&self) -> Option<MatchingEdges> {
        //If there is no start an end points there is no need doing augmenting path
        if !self.should_do_augmenting_path() {
            return None;
        }

        for v in &self.s {
            let visited: &mut VertexMap<bool> = &mut VertexMap::default();
            let path = &mut Vec::new();
            self.dfs(*v, visited, path);
            let end_v = path.last().unwrap();

            if self.t.contains(end_v) {
                let m = self.get_new_matching_edges(path);
                return Some(m);
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
        aug_path.out.entry(2).or_default().insert(10);
        aug_path.out.entry(6).or_default().insert(11);
        aug_path.out.entry(7).or_default().insert(12);
        aug_path.out.entry(9).or_default().insert(13);

        let edges = aug_path.get_new_matching_edges(&mut path);

        assert!(edges.e_add.contains_key(&10));
        assert!(edges.e_remove.contains_key(&3));
    }

    #[test]
    fn test_find_augmenting_path_should_return_edges_in_matching() {
        let mut aug_path = AugmentingPath::new(1);
        aug_path.s.extend([2, 6]);
        aug_path.t.extend([7, 9]);
        aug_path.out.entry(2).or_default().insert(10);
        aug_path.out.entry(6).or_default().insert(11);
        aug_path.out.entry(7).or_default().insert(12);
        aug_path.out.entry(9).or_default().insert(13);

        let edges = [(2, 3), (3, 4), (4, 5), (5, 6), (6, 7), (7, 8), (8, 9)];
        for (v, u) in edges {
            aug_path.edges.insert(v, u);
        }

        let path = aug_path.find_augmenting_path();

        assert!(path.is_some());
    }
}
