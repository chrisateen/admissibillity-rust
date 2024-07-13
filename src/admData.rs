use graphbench::graph::{Vertex, VertexMap, VertexSet};

pub struct AdmData {
    pub id: Vertex,
    pub n1_in_l: VertexSet,
    pub n1_in_r: VertexSet,
    pub deleted_m: bool,
    pub m: VertexMap<Vertex>, //key vertex v in M and in L, value neighbour of v in M and in R
    pub vias: VertexSet,
}

impl AdmData {
    pub fn new(v: Vertex, v_neighbours:VertexSet) -> Self{
        AdmData {
            id: v,
            n1_in_l: v_neighbours,
            n1_in_r: VertexSet::default(),
            deleted_m: false,
            m:VertexMap::default(),
            vias: VertexSet::default(),
        }
    }

    //When a vertex is added to candidates we no longer need M
    pub fn delete_m(&mut self){
        self.m = VertexMap::default();
        self.deleted_m = true;
    }

    pub fn is_maximal_matching_size_p(&self, p:usize) -> bool {
        return self.m.len() + self.n1_in_l.len() < p + 1;
    }

    pub fn move_n1_in_l_to_r(&mut self, v:&Vertex){
        self.n1_in_l.remove(v);
        self.n1_in_r.insert(*v);
    }

    //Checks if a vertex v in L is not in M or not in L1
    //If not that vertex can be added to M
    pub fn can_add_vertex_in_l_to_m(&self, v:&Vertex) -> bool{
        return !(self.m.contains_key(v) || self.n1_in_l.contains(v) || v.clone() == self.id)
    }

    pub fn initialise_vias(&mut self) {
        self.vias.extend(&self.n1_in_r);
        self.vias.extend(self.m.values());
    }
}

#[cfg(test)]
mod test_adm_data {
    use crate::admData::AdmData;

    #[test]
    fn delete_m_should_reset_m() {
        let neighbours = vec![2, 3, 4, 5].iter().cloned().collect();
        let mut v = AdmData::new(1,neighbours);
        v.m.insert(6,7);
        v.m.insert(9,10);
        v.delete_m();

        assert!(v.deleted_m);
        assert_eq!(v.m.len(), 0);
    }

    #[test]
    fn is_maximal_matching_size_p_should_check_if_len_l_and_m_is_p(){
        let neighbours = vec![2, 3, 4, 5].iter().cloned().collect();
        let mut v = AdmData::new(1,neighbours);
        v.m.insert(6,7);
        v.m.insert(9,10);

        assert!(!v.is_maximal_matching_size_p(5));
        assert!(v.is_maximal_matching_size_p(6));
        assert!(v.is_maximal_matching_size_p(7));
    }

    #[test]
    fn move_n1_in_l_to_r_should_remove_vertex_in_l_and_add_to_r(){
        let neighbours = vec![2, 3, 4, 5].iter().cloned().collect();
        let mut v = AdmData::new(1,neighbours);

        v.move_n1_in_l_to_r(&2);

        assert_eq!(v.n1_in_l.len(), 3);
        assert_eq!(v.n1_in_r.len(), 1);
    }

    #[test]
    fn can_add_vertex_in_l_to_m_returns_false_if_v_is_in_l(){
        let neighbours = vec![2, 3, 4, 5].iter().cloned().collect();
        let mut v = AdmData::new(1,neighbours);

        assert!(!v.can_add_vertex_in_l_to_m(&2));
    }

    #[test]
    fn can_add_vertex_in_l_to_m_returns_false_if_v_has_same_id_as_self(){
        let neighbours = vec![2, 3, 4, 5].iter().cloned().collect();
        let mut v = AdmData::new(1,neighbours);

        assert!(!v.can_add_vertex_in_l_to_m(&1));
    }

    #[test]
    fn can_add_vertex_in_l_to_m_returns_false_if_v_is_in_m() {
        let neighbours = vec![2, 3, 4, 5].iter().cloned().collect();
        let mut v = AdmData::new(1,neighbours);
        v.m.insert(6,7);

        assert!(!v.can_add_vertex_in_l_to_m(&6));
    }

    #[test]
    fn can_add_vertex_in_l_to_m_returns_false_if_v_is_not_in_m_or_l() {
        let neighbours = vec![2, 3, 4, 5].iter().cloned().collect();
        let mut v = AdmData::new(1,neighbours);
        v.m.insert(6,7);

        assert!(v.can_add_vertex_in_l_to_m(&8));
    }

    #[test]
    fn initialise_vias_should_add_vertices_in_r(){
        let l1 = vec![2, 3, 4, 5].iter().cloned().collect();
        let r1 = vec![6,7,8].iter().cloned().collect();
        let mut v = AdmData::new(1, l1);
        v.n1_in_r = r1;

        v.m.insert(9,10);
        v.m.insert(11,12);

        v.initialise_vias();

        assert_eq!(v.vias, vec![6,7,8,10,12].iter().cloned().collect());

    }
}