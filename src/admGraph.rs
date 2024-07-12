use graphbench::editgraph::EditGraph;
use graphbench::graph::{Graph, Vertex, VertexMap, VertexSet};
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

    //When a vertex v is moving into R remove v as an L1 from all of its neighbours
    //check if v can be added to M
    fn update_n1_of_v(&mut self, v:&AdmData){
        for u in self.graph.neighbours(&v.id){
            let u_adm_data = self.adm_data.get_mut(u).unwrap();
            u_adm_data.n1_in_l.remove(&v.id);
            u_adm_data.n1_in_r.insert(v.id);

            if !u_adm_data.deleted_m{
                for w in &v.n1_in_l{
                    if u_adm_data.can_add_vertex_in_l_to_m(w){
                        u_adm_data.m.insert(*w, v.id);
                    }
                }
            }
        }
    }

    //When a vertex v is moving into R for each of v's L2 neighbours replace/remove v from M
    fn update_l2_of_v(&mut self, v:&AdmData){
        let l_in_m: Vec<&Vertex>  = v.m.keys().collect();

        for u in l_in_m{
            let u_adm_data = &mut self.adm_data.get_mut(u).unwrap();

            //remove v from m of u if v is in m of u
            let v_neighbour_in_m = u_adm_data.m.remove(&v.id);

            match v_neighbour_in_m {
                None => return,
                //check to see if we can replace the edge being removed
                Some(x) => {
                    let x_adm_data = self.adm_data.get(&x).unwrap();
                    //check if there is another vertex that can replace v
                    for y in &x_adm_data.n1_in_l {
                        if u_adm_data.can_add_vertex_in_l_to_m(y) {
                            u_adm_data.m.insert(*y, x);
                            break;
                        }
                    }
                }
            }

        }
    }

    fn add_vias(&mut self, p:usize){
        // let mut counter : VertexMap<usize> = VertexMap::default();
        // let neighbours_v = graph.neighbours(&self.id);
        //
        // let _ = self.n1_in_l.iter().map(|x| self.vias.insert(*x));
        // let _ = self.m.values().map(|x| self.vias.insert(x.id.clone()));
        //
        // //For each L in M count how many neighbours it has in R and M
        // for (u,_) in &self.m {
        //     for w in self.m.values(){
        //         if w.n1_in_l.contains(&u){
        //             *counter.entry(*u).or_default() += 1;
        //         }
        //     }
        // }
        //
        // for w in neighbours_v{
        //     if !(self.m.contains_key(&w) && self.n1_in_l.contains(&w) ){
        //         for (u,_) in &self.m {
        //             if graph.adjacent(u,w){
        //                 *counter.entry(*u).or_default() += 1;
        //
        //                 if counter.get(u).unwrap() <= &(p + 1) {
        //                     self.vias.insert(*w);
        //                 }
        //             }
        //         }
        //     }
        // }
    }

    fn move_vertex_to_r(&mut self, p: usize){
        
    }

    //TODO
    fn construct_g_for_augmenting_path(&self) {

    }

    //TODO DFS
    fn augmenting_path(){
    }

}

#[cfg(test)]
mod test_adm_graph {
    //TODO
}