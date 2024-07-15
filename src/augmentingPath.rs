use graphbench::graph::{Vertex, VertexMap, VertexSet};

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

    fn get_new_matching_edges(&self, path: &mut Vec<Vertex>) -> VertexMap<Vertex> {
        let mut m: VertexMap<Vertex> = VertexMap::default();

        let first = path.remove(0);
        m.insert(*self.out.get(&first).unwrap().iter().next().unwrap(), first);

        let last = path.remove(path.len() - 1);
        m.insert(last, *self.out.get(&last).unwrap().iter().next().unwrap());

        for c in path.chunks(2) {
            m.insert(*c.get(0).unwrap(), *c.get(1).unwrap());
        }

        m
    }

    pub fn find_augmenting_path(&self, p: usize) -> Option<VertexMap<Vertex>> {
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
                if m.len() == p + 1 {
                    println!("{:?}", path);
                    println!("Len = {}, P = {}", m.len(), p);
                    return Some(m);
                }
            }
        }

        return None
    }
}

#[cfg(test)]
mod test_augmenting_path {
    use graphbench::graph::Vertex;
    use crate::augmentingPath::AugmentingPath;

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
    fn test_get_new_matching_edges_should_return_edges_in_matching(){
        let mut aug_path = AugmentingPath::new(1);
        aug_path.s.extend([2, 6]);
        aug_path.t.extend([7, 9]);
        let mut path: Vec<Vertex> = Vec::new();
        [2,3,4,5,6,7,8,9].map(|x| {path.push(*&x)});
        aug_path.out.entry(2).or_default().insert(10);
        aug_path.out.entry(6).or_default().insert(11);
        aug_path.out.entry(7).or_default().insert(12);
        aug_path.out.entry(9).or_default().insert(13);

        let actual = aug_path.get_new_matching_edges(&mut path);

        assert_eq!(actual.contains_key(&10), true);
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

        let path = aug_path.find_augmenting_path(4).unwrap();

        assert_eq!(path.len(), 5);
        assert!(path.contains_key(&10));
        assert!(path.contains_key(&9));
        assert!(path.contains_key(&3));
    }

    #[test]
    fn test_find_augmenting_path_should_none_if_matching_is_p_or_less() {
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

        let path = aug_path.find_augmenting_path(5);

        assert!(path.is_none());
    }
}
