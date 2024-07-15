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
                        break;
                    }
                }
            }
        }
    }

    //When a vertex v is moving into R for each of v's L2 neighbours replace/remove v from M
    fn update_l2_of_v(&mut self, v:&AdmData){
        let edges_in_m_v: Vec<(&Vertex,&Vertex)>  = v.m.iter().map(|x| x ).collect();

        for (u, x) in edges_in_m_v{
            let x_adm_data = self.adm_data.get_mut(x).unwrap();

            //As v is moving to r, need to move v as an R1 neighbour of x
            x_adm_data.move_n1_in_l_to_r(&v.id);

            let x_n1_in_l = x_adm_data.n1_in_l.clone();
            let u_adm_data = &mut self.adm_data.get_mut(u).unwrap();

            //check if v is in m of u and if so remove
            match u_adm_data.m.remove(&v.id){
                None => continue,
                //check to see if we can replace the edge x,v being removed
                //by checking if v can be replaced by another vertex in L1 of x
                Some(x) => {
                    for y in &x_n1_in_l {
                        if u_adm_data.can_add_vertex_in_l_to_m(y) {
                            u_adm_data.m.insert(*y, x);
                            break;
                        }
                    }
                }
            }
        }
    }

    fn add_vias(&mut self, v: &mut AdmData, p:usize){
        let mut counter : VertexMap<usize> = VertexMap::default();
        let neighbours_v = self.graph.neighbours(&v.id);

        v.initialise_vias();

        //For each vertex in L of v & M of v
        //count how many neighbours it has in R of v and in M of v
        for (u,_) in &v.m {
            for w in v.m.values(){
                let w_adm_data = self.adm_data.get(w).unwrap();
                if w_adm_data.n1_in_l.contains(&u){
                    *counter.entry(*u).or_default() += 1;
                }
            }
        }
        
        for w in neighbours_v{
            if !(v.m.contains_key(&w) || v.n1_in_l.contains(&w) ){
                for (u,_) in &v.m {
                    if self.graph.adjacent(u,w){
                        *counter.entry(*u).or_default() += 1;

                        if counter.get(u).unwrap() <= &(p + 1) {
                            v.vias.insert(*w);
                        }
                    }
                }
            }
        }
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