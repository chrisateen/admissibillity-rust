use graphbench::editgraph::EditGraph;
use graphbench::graph::{Graph, VertexMap, VertexSet};
use crate::admData::AdmData;

pub struct AdmGraph {
    graph: EditGraph,
    l: VertexSet,
    r: VertexSet,
    candidates: VertexSet,
    adm_data: VertexMap<AdmData>
}

impl AdmGraph {
    pub fn new(graph: EditGraph) -> Self{
        let mut adm_data = VertexMap::default();
        let l = graph.vertices().copied().collect();
        for u in graph.vertices(){
            //for each vertex in the graph get all of its neighbours
            let adm_vertex = AdmData::new(*u, graph.neighbours(u).copied().collect());
            adm_data.insert(*u,adm_vertex);
        }
        AdmGraph{graph, l , r: VertexSet::default(), candidates: VertexSet::default() , adm_data}
    }

    //TODO enable initialisation of candidates from previous iterations
    pub fn initialise_candidates(&mut self, p: usize){
        for (u, adm_data) in &self.adm_data{
            if adm_data.n1_in_l.len() <= p {
                self.candidates.insert(*u);
            }
        }
    }

    pub fn move_vertex_to_r(&mut self, p: usize){
        
    }

}