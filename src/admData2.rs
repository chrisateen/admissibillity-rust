use std::collections::HashSet;
use graphbench::graph::{Vertex, VertexMap, VertexSet};

pub struct InternalT2 {
    pub external_vertices: VertexSet,
    pub matching_vertex: VertexSet,
    pub internal_vertices: VertexSet
}

pub struct InternalVia {
    pub matching_vertex: VertexSet,
    pub internal_vertices: VertexSet,
    pub external_vertices: VertexSet,
}

pub struct AdmData {
    id: Vertex,
    pub estimate: usize,
    pub t1: VertexSet,
    pub t2: VertexSet,
    pub internal_t2: VertexMap<InternalT2>,
    pub internal_via: VertexMap<InternalVia>,
}

impl AdmData {
    pub fn new(id: Vertex, t1:VertexSet) -> Self{
       AdmData {
           id,
           estimate: t1.len(),
           t1,
           t2: VertexSet::default(),
           internal_t2: VertexMap::default(),
           internal_via: VertexMap::default()
       }
    }

    pub fn add_vertices(&mut self, v:&Vertex, v_neighbours:&VertexSet){
        let new_t2 = (&v_neighbours - &self.t2).iter().cloned().collect();
    }
}