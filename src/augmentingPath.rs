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
        self.s.len() > 0 && self.t.len() > 0
    }

    fn dfs(&self, v: Vertex, visited: &mut VertexMap<bool>, path: &mut Vec<Vertex>) {
        visited.insert(v,true);
        path.push(v);

        match self.edges.get(&v){
            None => return,
            Some(v_neighbour) =>{
                match visited.get(v_neighbour){
                    None => {
                        self.dfs(*v_neighbour, visited, path);
                    }
                    Some(has_visited) => {
                        if !has_visited{
                            self.dfs(*v_neighbour, visited, path);
                        }
                    }
                }
            }
        }
    }

    pub fn find_augmenting_path(&self, p:usize) -> Option<Vec<Vertex>>{
        let mut longest_path : Vec<Vertex> = Vec::new();

        for v in &self.s{
            let visited : &mut VertexMap<bool> = &mut VertexMap::default();
            let path = &mut Vec::new();
            self.dfs(*v, visited, path);
            if longest_path.len() < path.len(){
                longest_path = path.clone();
                println!("{longest_path:?}");
            }
        }

        if longest_path.len() == p*2{
            Some(longest_path)
        }else{
            None
        }
    }
}

#[cfg(test)]
mod test_augmenting_path {
    use crate::augmentingPath::AugmentingPath;

    #[test]
    fn test_should_do_augmenting_path_should_return_false_if_s_and_t_is_empty() {
        let mut aug_path = AugmentingPath::new(1);
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
    fn test_find_augmenting_path() {
        let mut aug_path = AugmentingPath::new(1);
        aug_path.s.extend([2,6]);
        aug_path.t.extend([7,9]);
        aug_path.out.entry(2).or_default().insert(10);
        aug_path.out.entry(6).or_default().insert(11);
        aug_path.out.entry(7).or_default().insert(12);
        aug_path.out.entry(9).or_default().insert(13);

        let _ = [(2, 3), (3, 4), (4, 5), (5, 6), (6, 7), (7, 8), (8, 9)]
            .iter()
            .map(|(u,v)| {aug_path.edges.insert(*u, *v)});

        aug_path.find_augmenting_path(4);
    }
}
