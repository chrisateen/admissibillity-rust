use graphbench::editgraph::EditGraph;
use graphbench::graph::*;
use crate::AdmData;

pub struct AdmGraph<'a> {
    graph: &'a EditGraph,
    ordering: Vec<Vertex>,
    unordered: VertexSet,
    adm_data: VertexMap<AdmData>
}

impl<'a> AdmGraph<'a> {
    pub fn new(graph:&'a EditGraph) -> Self{
        let mut adm_data = VertexMap::default();
        let unordered = graph.vertices().copied().collect();
        for u in graph.vertices(){
            //for each vertex in the graph get all of its neighbours
            let adm_vertex = AdmData::new(*u, graph.neighbours(u).copied().collect());
            adm_data.insert(*u,adm_vertex);
        }
        AdmGraph{graph,ordering: Vec::new(), unordered, adm_data}
    }

    pub fn compute_ordering(&mut self, p: usize) -> bool{
        let mut candidates = VertexSet::default();
        // Gets all vertices in the graph with the number of neighbours less than or equal to p
        for (u, adm_data) in &self.adm_data{
            if adm_data.layer_1.len() <= p {
                candidates.insert(*u);
            }
        }

        while !candidates.is_empty(){
            //We know p is true if all the vertices have neighbours less than or equal to p
            if candidates.len() + self.ordering.len() == self.graph.num_vertices() { true; }

            // Remove a vertex from the list of candidates and add to the ordering
            let u = *candidates.iter().next().unwrap();
            candidates.remove(&u);
            self.ordering.push(u);
            self.unordered.remove(&u);

            let u_data = self.adm_data.remove(&u).unwrap();

            // Update the layer 1 and layer 2 vertices for each left neighbours of u
            for v in &u_data.layer_1{
                let v_data = self.adm_data.get_mut(&v).unwrap_or_else(|| panic!("{v} is not contained in {u} layer 1"));
                v_data.update_direct_neighbour(&u, &u_data.layer_1, p);
                // Add v to candidates if estimate is p or less
                if v_data.estimate <= p {
                    candidates.insert(*v);
                }
            }
            // Check all the layer 2 vertices (v).
            // If u is listed as a layer 2 vertex of v remove it as v is now to the right of u
            for v in u_data.layer_2.keys(){
                let v_data = self.adm_data.get_mut(&v).unwrap();
                v_data.update_indirect_neighbour(&u,p);
                if v_data.estimate <= p {
                    candidates.insert(*v);
                }
            }
        }
        //If all vertices are ordered then p is true
        self.ordering.len() == self.graph.num_vertices()
    }
}


#[cfg(test)]
mod test_adm_graph {
    use graphbench::graph::*;
    use graphbench::editgraph::EditGraph;
    use crate::admGraph::AdmGraph;

    #[test]
    pub fn compute_ordering_returns_true_if_all_v_in_g_has_neighbours_on_or_below_p(){
        let mut graph = EditGraph::new();
        let edges: EdgeSet =  vec![(1, 2), (1, 3), (1,4), (1,5), (2,6), (3,6), (4,6), (5,6)].iter().cloned().collect();
        for (u,v) in edges.iter(){
            graph.add_edge(u,v);
        }
        let num_vertices = graph.num_vertices();
        let mut adm_graph = AdmGraph::new(&graph);
        assert_eq!(adm_graph.compute_ordering(4),true);
        assert_eq!(adm_graph.unordered.is_empty(),true);
        assert_eq!(adm_graph.ordering.len(),num_vertices);
    }

    #[test]
    pub fn compute_ordering_returns_true_for_correct_p_value(){
        let mut graph = EditGraph::new();
        let edges: EdgeSet =  vec![(1, 2), (1, 3), (1,4), (2,3), (2,4), (3,4)].iter().cloned().collect();
        for (u,v) in edges.iter(){
            graph.add_edge(u,v);
        }
        let num_vertices = graph.num_vertices();
        let mut adm_graph = AdmGraph::new(&graph);
        assert_eq!(adm_graph.compute_ordering(3),true);
        assert_eq!(adm_graph.unordered.is_empty(),true);
        assert_eq!(adm_graph.ordering.len(),num_vertices);
    }

    #[test]
    pub fn compute_ordering_returns_false_for_incorrect_p_value(){
        let mut graph = EditGraph::new();
        let edges: EdgeSet =  vec![(1, 2), (1, 3), (1,4), (2,3), (2,4), (3,4)].iter().cloned().collect();
        for (u,v) in edges.iter(){
            graph.add_edge(u,v);
        }
        let num_vertices = graph.num_vertices();
        let mut adm_graph = AdmGraph::new(&graph);
        assert_eq!(adm_graph.compute_ordering(2),false);
        assert_eq!(adm_graph.unordered.is_empty(),false);
        assert!(adm_graph.ordering.len() < num_vertices);
    }
}