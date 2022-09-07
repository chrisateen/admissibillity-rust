#![allow(unused)]
use graphbench::graph::*;
use graphbench::editgraph::EditGraph;
use graphbench::iterators::EdgeIterable;
use std::fs;
use std::env;
use std::path::{PathBuf};
use std::fs::{copy, File};
use flate2::read::GzDecoder;
use std::io::{BufReader, Read, Write};
use graphbench::algorithms::GraphAlgorithms;
use hopcroft_karp::matching;


struct AdmData {
    id: Vertex,
    layer_1: VertexSet,
    layer_2: VertexMap<VertexSet>,
    estimate: usize,
    //num_layer_2_vias: usize,//change this to a set instead
}

impl AdmData {
    pub fn new(id: Vertex, layer_1:VertexSet) -> Self{
        AdmData {id, estimate: layer_1.len(), layer_1, layer_2: VertexMap::default()}
    }

    pub fn check_integrity(&self, graph:&AdmGraph) {
        for u in &self.layer_1{
            debug_assert!(graph.unordered.contains(&u));
        }

        for (u, vias) in &self.layer_2{
            debug_assert!(graph.unordered.contains(&u));
            for v in vias{
                debug_assert!(graph.ordering.contains(&v));
            }
        }

        debug_assert!(graph.unordered.contains(&self.id));
    }

    // fn is_num_layer_2_above_threshold(&self, p:usize) -> bool{
    //     self.layer_2.len() > (p - self.layer_1.len()) * (p - 1)
    // }
    //
    // fn is_num_of_vias_above_threshold(&self, p:usize) -> bool{
    //     let num_layer_1 = self.layer_1.len();
    //     self.num_layer_2_vias > (p - num_layer_1) * (p - num_layer_1 + 1)
    // }

    // Compute packing for the layer 2 vertices
    // Return the maximum matching value
    pub fn compute_packing(&self) -> usize {
        let mut edges = Vec::default();
        let len_layer_2 = self.layer_2.len();
        if len_layer_2 <= 1 {
            return len_layer_2
        }
        for (u, vias) in &self.layer_2{
            edges.extend(vias.iter().map(|v| (u,v)));
        }
        println!("{:?}",edges);
        matching(&edges).len()
    }

    // Update the estimate when the current estimate is p
    pub fn update_estimate(&mut self, p: usize) {
        if self.estimate != p{
            return;
        }
        // Don't do maximum matching if num of layer 2 or vias is above threshold
        // Instead increment estimate by 1
        // if self.is_num_layer_2_above_threshold(p) || self.is_num_of_vias_above_threshold(p){
        //     self.estimate += 1;
        //     return;
        // }
        self.estimate = self.layer_1.len() + self.compute_packing();
    }

    pub fn update_indirect_neighbour(&mut self, u:&Vertex, p:usize){
        self.layer_2.remove(&u);
        //As we are losing a layer 2 vertex we want to reduce the estimate
        self.estimate = self.estimate.saturating_sub(1);
        self.update_estimate(p);
    }

    // Remove u from list of layer_1 vertices as it's is now to the right of AdmData
    // Adds all u_layer1 vertices to layer_2 with u being the via
    pub fn update_direct_neighbour(&mut self, u:&Vertex, u_layer1:&VertexSet, p:usize){
        // Remove the vertex that is now to the right
        self.layer_1.remove(u);
        self.estimate = self.estimate.saturating_sub(1);
        for v in u_layer1{
            if *v == self.id || self.layer_1.contains(v){
                continue
            }
            // Only add v as a layer_2 vertex if it is not directly reachable
            let entry = self.layer_2.entry(*v).or_default();
            // Only add u as a via if v has less than p + 1 vias
            if entry.len() < p + 1 {
                entry.insert(*u);
            }
        }
        self.update_estimate(p);
    }

}


struct AdmGraph{
    graph: EditGraph,
    ordering: Vec<Vertex>,
    unordered: VertexSet,
    adm_data: VertexMap<AdmData>
}

impl AdmGraph{
    pub fn new(graph:EditGraph) -> Self{
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

            for (u, u_data) in &self.adm_data{
                u_data.check_integrity(self);
            }

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
                let v_data = self.adm_data.get_mut(&v).unwrap();
                v_data.update_direct_neighbour(&u, &u_data.layer_1, p);
                // Add v to candidates if estimate is p or less
                if v_data.estimate <= p {
                    candidates.insert(*v);
                }
            }
            for v in &u_data.layer_1{
                let v_data = self.adm_data.get(&v).unwrap();
                v_data.check_integrity(&self);
            }
            for v in u_data.layer_2.keys(){
                let v_data = self.adm_data.get_mut(&v).unwrap();
                v_data.update_indirect_neighbour(&u,p);
                if v_data.estimate <= p {
                    candidates.insert(*v);
                }
            }
            for v in u_data.layer_2.keys(){
                let v_data = self.adm_data.get(&v).unwrap();
                v_data.check_integrity(&self);
            }
        }
        //If all vertices are ordered then p is true
        self.ordering.len() == self.graph.num_vertices()
    }
}

fn main() {
    let mut graph = EditGraph::from_gzipped("../network-corpus/networks/windsurfers.txt.gz").expect("Could not open edges.txt");
    // let (lower, upper, ordering, core_numbers)  = graph.degeneracy();
    // println!("Degeneracy: {:?}", lower);
    let mut adm_graph = AdmGraph::new(graph);
    let p = 16;
    let is_p = adm_graph.compute_ordering(p);
    println!(" Is {0}-2 Admissible:{1}", p, is_p);
}
