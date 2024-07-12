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

    //Checks if a vertex v in L is not in M or not in L1
    //If not that vertex can be added to M
    pub fn can_add_vertex_in_l_to_m(&self, v:&Vertex) -> bool{
        return !self.m.contains_key(v) || !self.n1_in_l.contains(v) || v.clone() != self.id
    }
}

#[cfg(test)]
mod test_adm_data {
    //TODO
}