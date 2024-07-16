use std::mem;
use crate::admData::AdmData;
use crate::augmentingPath::AugmentingPath;
use graphbench::editgraph::EditGraph;
use graphbench::graph::{Graph, Vertex, VertexMap, VertexSet};

pub struct AdmGraph<'a> {
    graph: &'a EditGraph,
    l: VertexSet,
    r: VertexSet,
    checks: VertexSet,
    pub candidates: VertexSet,
    adm_data: VertexMap<AdmData>,
}

impl<'a> AdmGraph<'a> {
    pub fn new(graph: &'a EditGraph) -> Self {
        let mut adm_data = VertexMap::default();
        let l = graph.vertices().copied().collect();
        for u in graph.vertices() {
            let adm_vertex = AdmData::new(*u, graph.neighbours(u).copied().collect());
            adm_data.insert(*u, adm_vertex);
        }
        AdmGraph {
            graph,
            l,
            r: VertexSet::default(),
            checks: VertexSet::default(),
            candidates: VertexSet::default(),
            adm_data,
        }
    }

    //TODO enable initialisation of candidates from previous iterations
    pub fn initialise_candidates(&mut self, p: usize) {
        for (u, adm_data) in &self.adm_data {
            if adm_data.n_in_l.len() <= p {
                self.candidates.insert(*u);
            }
        }
    }

    //When a vertex v is moving into R need to move v from L to R for all of v's neighbours u in L
    //check if v can be added to M of u
    fn update_n1_of_v(&mut self, v: Vertex) {
        let n_in_l = self.adm_data.get(&v).unwrap().n_in_l.clone();
        for u in self.graph.neighbours(&v) {
            let u_adm_data = self.adm_data.get_mut(u).unwrap();
            u_adm_data.move_v_in_l_to_r(&v);

            if !u_adm_data.deleted_m {
                for w in &n_in_l {
                    if u_adm_data.can_add_vertex_in_l_to_m(w) {
                        u_adm_data.add_edges_to_m(*w, v);
                        break;
                    }
                }
                if !self.candidates.contains(u) {
                    self.checks.insert(*u);
                }
            }
        }
    }

    fn union_left_neighbours_of_u_in_r_and_m(&self, v:Vertex) -> VertexSet{
        let mut union_left_neighbours = VertexSet::default();
        let v_adm_data = self.adm_data.get(&v).unwrap();

        for x in v_adm_data.m_from_r.keys() {
            let v_adm_data = self.adm_data.get(x).unwrap();
            union_left_neighbours.extend(v_adm_data.n_in_l.iter().cloned());
        }
        union_left_neighbours
    }

    //When a vertex v is moving into R for each vertex that is within 2 left neighbours of v
    // replace/remove v from M
    fn update_l2_of_v(&mut self, v: Vertex) {
        let union_left_neighbours = self.union_left_neighbours_of_u_in_r_and_m(v);

        for u in union_left_neighbours {
            let mut u_adm_data = self.adm_data.remove(&u).unwrap();
            //check if v is in m of u and if so remove
            match u_adm_data.remove_v_from_m(v) {
                None => {}
                //check to see if we can replace the edge x,v being removed
                //by checking if v can be replaced by another vertex in L1 of x
                Some(x) => {
                    let x_n_in_l: VertexSet = self
                        .adm_data
                        .get(&x)
                        .unwrap()
                        .n_in_l
                        .iter()
                        .cloned()
                        .collect();
                    for y in x_n_in_l {
                        if u_adm_data.can_add_vertex_in_l_to_m(&y) {
                            u_adm_data.add_edges_to_m(y, x);
                            break;
                        }
                    }
                }
            }
            self.adm_data.insert(u, u_adm_data);

            if !self.candidates.contains(&u) {
                self.checks.insert(u);
            }
        }
    }

    fn construct_g_for_augmenting_path(&self, v: &mut AdmData) -> AugmentingPath {
        let mut augmenting_path = AugmentingPath::new(v.id);

        //Get all the edges between vertices in r and vertices in L & M
        for u in &v.n_in_r {
            for (w, w_neighbour_in_m) in &v.m_from_l {
                //Gets matching edges already in M
                if *u == *w_neighbour_in_m {
                    augmenting_path.edges.entry(*u).or_default().insert(*w);
                } else if self.graph.adjacent(u, w) {
                    //Gets edges between vertices in M (excluding matching edges)
                    if v.m_from_r.contains_key(u) {
                        augmenting_path.edges.entry(*w).or_default().insert(*u);
                    } else {
                        //Gets edges between a vertex in L and M and a vertex R not in M
                        augmenting_path.t.insert(*w);
                        augmenting_path.out.insert(*w,*u);
                    }
                }
            }
        }

        //Gets edges between vertex u in R & M and vertex w in L but not in M
        for u in v.m_from_r.keys() {
            let u_adm_data = self.adm_data.get(u).unwrap();
            for w in &u_adm_data.n_in_l {
                if !v.m_from_l.contains_key(w) && !v.n_in_l.contains(w) && v.id != *w {
                    augmenting_path.s.insert(*u);
                    augmenting_path.out.insert(*u, *w);
                }
            }
        }
        augmenting_path
    }

    fn do_checks(&mut self, p: usize) {
        let checks = mem::take(&mut self.checks);
        for v in checks {
            let mut v_adm_data = self.adm_data.remove(&v).unwrap();
            if v_adm_data.is_maximal_matching_size_p(p) {
                let aug_path = self.construct_g_for_augmenting_path(&mut v_adm_data);
                let new_path = aug_path.find_augmenting_path();

                match new_path {
                    Some(path) => {
                        assert_eq!(v_adm_data.n_in_l.len() + v_adm_data.m_from_l.len(), p);
                        v_adm_data.update_m(&path);
                        assert_eq!(v_adm_data.n_in_l.len() + v_adm_data.m_from_l.len(), p+1);
                        assert_eq!(v_adm_data.n_in_l.len() + v_adm_data.m_from_r.len(), p+1);
                    }
                    None => {
                        self.candidates.insert(v);
                        assert!(v_adm_data.n_in_l.len() + v_adm_data.m_from_l.len() < p+1);
                        assert!(v_adm_data.n_in_l.len() + v_adm_data.m_from_r.len() < p+1);
                    }
                }
            }
            self.adm_data.insert(v, v_adm_data);
        }
    }

    pub fn remove_v_from_candidates(&mut self, p: usize) -> Option<Vertex> {
        let v = self.candidates.iter().next();

        match v {
            Some(&v) => {
                self.candidates.remove(&v);
                self.l.remove(&v);
                self.r.insert(v);

                self.update_n1_of_v(v);
                self.update_l2_of_v(v);

                self.do_checks(p);

                self.adm_data.get_mut(&v).unwrap().delete_m();

                Some(v)
            }
            None => None,
        }
    }
}

#[cfg(test)]
mod test_adm_graph {
    use crate::admGraph::AdmGraph;
    use graphbench::editgraph::EditGraph;
    use graphbench::graph::{EdgeSet, MutableGraph};

    #[test]
    fn initialise_candidates_should_add_vertices_with_degree_p_or_less_to_candidates() {
        let mut graph = EditGraph::new();
        let edges: EdgeSet = [(1, 2), (1, 3), (1, 4), (2, 5), (2, 6), (3, 7)]
            .iter()
            .cloned()
            .collect();
        for (u, v) in edges.iter() {
            graph.add_edge(u, v);
        }
        let mut adm_graph = AdmGraph::new(&graph);

        adm_graph.initialise_candidates(2);

        assert_eq!(
            adm_graph.candidates,
            [3, 4, 5, 6, 7].iter().cloned().collect()
        );
    }

    #[test]
    fn update_n1_of_v_should_move_v_to_m_of_u() {
        let mut graph = EditGraph::new();
        let edges: EdgeSet = [(1, 2), (1, 3), (1, 4), (2, 5), (2, 6)]
            .iter()
            .cloned()
            .collect();
        for (u, v) in edges.iter() {
            graph.add_edge(u, v);
        }
        let mut adm_graph = AdmGraph::new(&graph);

        adm_graph.update_n1_of_v(1);
        let u_adm_data = adm_graph.adm_data.get(&2).unwrap();

        assert_eq!(u_adm_data.m_from_l.len(), 1);
    }

    #[test]
    fn update_n1_of_v_should_move_v_to_m_of_u_if_m_of_u_has_been_deleted() {
        let mut graph = EditGraph::new();
        let edges: EdgeSet = [(1, 2), (1, 3), (1, 4), (2, 5), (2, 6)]
            .iter()
            .cloned()
            .collect();
        for (u, v) in edges.iter() {
            graph.add_edge(u, v);
        }
        let mut adm_graph = AdmGraph::new(&graph);

        let mut u_adm_data = adm_graph.adm_data.remove(&2).unwrap();
        u_adm_data.deleted_m = true;
        adm_graph.adm_data.insert(2, u_adm_data);

        adm_graph.update_n1_of_v(1);

        assert_eq!(adm_graph.adm_data.remove(&2).unwrap().m_from_l.len(), 0);
    }

    #[test]
    fn update_l2_of_v_should_remove_v_from_m_of_u_and_replace_edge() {
        let mut graph = EditGraph::new();
        let edges: EdgeSet = [(1, 2), (1, 3), (1, 4), (4, 5), (5, 6), (5, 7), (4, 8)]
            .iter()
            .cloned()
            .collect();
        for (u, v) in edges.iter() {
            graph.add_edge(u, v);
        }
        let mut adm_graph = AdmGraph::new(&graph);

        adm_graph.initialise_candidates(3);
        let v_adm_data = adm_graph.adm_data.get_mut(&1).unwrap();
        v_adm_data.move_v_in_l_to_r(&4);
        v_adm_data.add_edges_to_m(5, 4);
        let mut u_adm_data = adm_graph.adm_data.remove(&5).unwrap();
        u_adm_data.move_v_in_l_to_r(&4);
        u_adm_data.add_edges_to_m(1, 4);
        adm_graph.adm_data.insert(5, u_adm_data);

        adm_graph.update_l2_of_v(1);

        assert!(adm_graph
            .adm_data
            .get(&5)
            .unwrap()
            .m_from_l
            .contains_key(&8));
    }

    // #[test]
    // fn construct_g_for_augmenting_path_should_create_edges_for_augmenting_path() {
    //     let mut graph = EditGraph::new();
    //     let edges: EdgeSet = [
    //         (1, 2),
    //         (1, 3),
    //         (1, 4),
    //         (1, 6),
    //         (1, 8),
    //         (1, 10),
    //         (1, 11),
    //         (4, 5),
    //         (6, 7),
    //         (8, 9),
    //         (5, 6),
    //         (7, 8),
    //         (9, 10),
    //         (9, 11),
    //         (4, 12),
    //         (4, 13),
    //         (6, 14),
    //     ]
    //     .iter()
    //     .cloned()
    //     .collect();
    //     for (u, v) in edges.iter() {
    //         graph.add_edge(u, v);
    //     }
    //     let mut adm_graph = AdmGraph::new(&graph);
    //
    //     adm_graph.initialise_candidates(3);
    //     let mut v_adm_data = adm_graph.adm_data.remove(&1).unwrap();
    //     [4, 6, 8, 10, 11].map(|x| v_adm_data.move_v_in_l_to_r(&x));
    //     [(5, 4), (7, 6), (9, 8)].map(|(l, r)| v_adm_data.add_edges_to_m(l, r));
    //
    //     let aug_path = adm_graph.construct_g_for_augmenting_path(&mut v_adm_data);
    //
    //     assert_eq!(aug_path.t.len(), 1);
    //     assert!(aug_path.t.contains(&9));
    //     assert_eq!(aug_path.s.len(), 2);
    //     assert!(aug_path.s.contains(&4));
    //     assert!(aug_path.s.contains(&6));
    //     //Check some of the edges are inserted in the right direction
    //     assert_eq!(aug_path.edges.get(&4).unwrap(), &5);
    //     assert_eq!(aug_path.edges.get(&5).unwrap(), &6);
    //}

    #[test]
    fn remove_v_from_candidates_should_move_v_from_l_to_r() {
        let mut graph = EditGraph::new();
        let edges: EdgeSet = [(1, 2), (1, 3), (1, 4), (2, 5), (2, 6)]
            .iter()
            .cloned()
            .collect();
        for (u, v) in edges.iter() {
            graph.add_edge(u, v);
        }
        let mut adm_graph = AdmGraph::new(&graph);

        adm_graph.initialise_candidates(3);

        adm_graph.remove_v_from_candidates(3);

        assert_eq!(adm_graph.r.len(), 1);
        assert_eq!(adm_graph.l.len(), 5);
    }
}
