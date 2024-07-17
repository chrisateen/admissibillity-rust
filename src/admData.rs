use crate::augmentingPath::MatchingEdges;
use graphbench::graph::{Vertex, VertexMap, VertexSet};

pub struct AdmData {
    pub id: Vertex,
    pub n_in_l: VertexSet,
    pub n_in_r: VertexSet,
    pub deleted_m: bool,
    pub m_from_l: VertexMap<Vertex>, //key vertex v in M and in L, value neighbour of v in M and in R
    pub m_from_r: VertexMap<Vertex>, //key vertex v in M and in R, value neighbour of v in M and in L
}

impl AdmData {
    pub fn new(v: Vertex, v_neighbours: VertexSet) -> Self {
        AdmData {
            id: v,
            n_in_l: v_neighbours,
            n_in_r: VertexSet::default(),
            deleted_m: false,
            m_from_l: VertexMap::default(),
            m_from_r: VertexMap::default(),
        }
    }

    //When a vertex is added to candidates we no longer need M
    pub fn delete_m(&mut self) {
        self.m_from_l = VertexMap::default();
        self.m_from_r = VertexMap::default();
        self.deleted_m = true;
    }

    pub fn remove_v_from_m(&mut self, v: Vertex) -> Option<Vertex> {
        match self.m_from_l.remove(&v) {
            None => None,
            Some(u) => {
                self.m_from_r
                    .remove(&u)
                    .unwrap_or_else(|| panic!("Vertex {u} should be in M"));
                Some(u)
            }
        }
    }

    pub fn move_v_in_l_to_r(&mut self, v: &Vertex) {
        self.n_in_l.remove(v);
        self.n_in_r.insert(*v);
    }

    pub fn add_edges_to_m(&mut self, v_in_l: Vertex, u_in_r: Vertex) {
        self.m_from_r.insert(u_in_r, v_in_l);
        self.m_from_l.insert(v_in_l, u_in_r);
    }

    //Update m when an augmenting path has been found
    pub fn update_m(&mut self, edges: &MatchingEdges) {
        for (v, u) in &edges.e_remove {
            self.m_from_l.remove(v);
            self.m_from_r.remove(u);
        }
        for (v, u) in &edges.e_add {
            self.m_from_l.insert(*v, *u);
            self.m_from_r.insert(*u, *v);
        }
    }

    pub fn is_maximal_matching_size_p(&self, p: usize) -> bool {
        self.m_from_l.len() + self.n_in_l.len() < p + 1
    }

    //Checks if a vertex v in L is not in M or not in L1
    //If not that vertex can be added to M
    pub fn can_add_vertex_in_l_to_m(&self, v: &Vertex) -> bool {
        !(self.m_from_l.contains_key(v) || self.n_in_l.contains(v) || v.eq(&self.id))
    }
}

#[cfg(test)]
mod test_adm_data {
    use crate::admData::AdmData;
    use crate::augmentingPath::MatchingEdges;
    use graphbench::graph::VertexMap;

    #[test]
    fn delete_m_should_reset_m() {
        let neighbours = [2, 3, 4, 5].iter().cloned().collect();
        let mut v = AdmData::new(1, neighbours);
        v.m_from_l.insert(6, 7);
        v.m_from_l.insert(9, 10);
        v.m_from_r.insert(7, 6);
        v.m_from_r.insert(10, 9);

        v.delete_m();

        assert!(v.deleted_m);
        assert_eq!(v.m_from_l.len(), 0);
        assert_eq!(v.m_from_r.len(), 0);
    }

    #[test]
    fn remove_v_from_m_should_remove_v_as_an_l2_of_m() {
        let neighbours = [2, 3, 4, 5].iter().cloned().collect();
        let mut v = AdmData::new(1, neighbours);
        v.m_from_l.insert(6, 7);
        v.m_from_l.insert(9, 10);
        v.m_from_r.insert(7, 6);
        v.m_from_r.insert(10, 9);

        assert!(v.remove_v_from_m(6).is_some());
        assert!(!v.m_from_l.contains_key(&6));
        assert!(!v.m_from_r.contains_key(&7));
    }

    #[test]
    fn remove_v_from_m_should_return_none_if_v_is_not_in_m() {
        let neighbours = [2, 3, 4, 5].iter().cloned().collect();
        let mut v = AdmData::new(1, neighbours);
        v.m_from_l.insert(6, 7);
        v.m_from_l.insert(9, 10);
        v.m_from_r.insert(7, 6);
        v.m_from_r.insert(10, 9);

        assert!(v.remove_v_from_m(0).is_none());
    }

    #[test]
    fn move_v_in_l_to_r_should_remove_vertex_in_l_and_add_to_r() {
        let neighbours = [2, 3, 4, 5].iter().cloned().collect();
        let mut v = AdmData::new(1, neighbours);

        v.move_v_in_l_to_r(&2);

        assert_eq!(v.n_in_l.len(), 3);
        assert_eq!(v.n_in_r.len(), 1);
    }

    #[test]
    fn add_edges_to_m_should_add_edges_in_both_directions() {
        let neighbours = [2, 3, 4, 5].iter().cloned().collect();
        let mut v = AdmData::new(1, neighbours);

        v.add_edges_to_m(6, 7);

        assert!(v.m_from_l.contains_key(&6));
        assert!(v.m_from_r.contains_key(&7));
    }

    #[test]
    fn update_m_should_add_and_remove_edges_in_m() {
        let neighbours = [2, 3, 4, 5].iter().cloned().collect();
        let mut v = AdmData::new(1, neighbours);
        v.m_from_l.insert(6, 7);
        v.m_from_r.insert(7, 6);

        let mut matching_edges = MatchingEdges {
            e_add: VertexMap::default(),
            e_remove: VertexMap::default(),
        };

        matching_edges.e_add.insert(10, 7);
        matching_edges.e_add.insert(6, 9);
        matching_edges.e_remove.insert(6, 7);

        v.update_m(&matching_edges);

        assert_eq!(v.m_from_l.len(), 2);
        assert_eq!(v.m_from_r.len(), 2);
        assert_eq!(*v.m_from_l.get(&10).unwrap(), 7);
        assert_eq!(*v.m_from_l.get(&6).unwrap(), 9);
        assert_eq!(*v.m_from_r.get(&7).unwrap(), 10);
        assert_eq!(*v.m_from_r.get(&9).unwrap(), 6);
    }

    #[test]
    fn is_maximal_matching_size_p_should_check_if_len_l_and_m_is_p() {
        let neighbours = [2, 3, 4, 5].iter().cloned().collect();
        let mut v = AdmData::new(1, neighbours);
        v.m_from_l.insert(6, 7);
        v.m_from_l.insert(9, 10);
        v.m_from_r.insert(7, 6);
        v.m_from_r.insert(10, 9);

        assert!(!v.is_maximal_matching_size_p(5));
        assert!(v.is_maximal_matching_size_p(6));
        assert!(v.is_maximal_matching_size_p(7));
    }

    #[test]
    fn can_add_vertex_in_l_to_m_returns_false_if_v_is_in_l() {
        let neighbours = [2, 3, 4, 5].iter().cloned().collect();
        let v = AdmData::new(1, neighbours);

        assert!(!v.can_add_vertex_in_l_to_m(&2));
    }

    #[test]
    fn can_add_vertex_in_l_to_m_returns_false_if_v_has_same_id_as_self() {
        let neighbours = [2, 3, 4, 5].iter().cloned().collect();
        let v = AdmData::new(1, neighbours);

        assert!(!v.can_add_vertex_in_l_to_m(&1));
    }

    #[test]
    fn can_add_vertex_in_l_to_m_returns_false_if_v_is_in_m() {
        let neighbours = [2, 3, 4, 5].iter().cloned().collect();
        let mut v = AdmData::new(1, neighbours);
        v.m_from_l.insert(6, 7);
        v.m_from_r.insert(7, 6);

        assert!(!v.can_add_vertex_in_l_to_m(&6));
    }

    #[test]
    fn can_add_vertex_in_l_to_m_returns_false_if_v_is_not_in_m_or_l() {
        let neighbours = [2, 3, 4, 5].iter().cloned().collect();
        let mut v = AdmData::new(1, neighbours);
        v.m_from_l.insert(6, 7);
        v.m_from_r.insert(7, 6);

        assert!(v.can_add_vertex_in_l_to_m(&8));
    }
}
