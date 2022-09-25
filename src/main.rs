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

    // Update the estimate
    pub fn update_estimate(&mut self, p: usize) {
        //Don't update the estimate if estimate is not p
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

    //Remove a layer 2 vertex
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
            // Only add v as a layer_2 vertex if it is not directly reachable
            if *v == self.id || self.layer_1.contains(v){
                continue
            }
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
            //for testing
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
            //for testing
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


#[cfg(test)]
mod test_adm_data {
    use std::collections::HashMap;
    use graphbench::editgraph::EditGraph;
    use graphbench::graph::{MutableGraph, VertexMap, VertexSet, Vertex};
    use crate::AdmData;

    #[test]
    fn update_estimate_does_not_update_if_estimate_is_not_p() {
        let layer_1 = vec![2, 3, 4, 5].iter().cloned().collect();
        let vias = vec![7, 8, 9].iter().cloned().collect();
        let mut adm_data = AdmData::new(1, layer_1);
        adm_data.estimate = 4;
        adm_data.layer_2.insert(6, vias);
        adm_data.update_estimate(3);
        assert_eq!(adm_data.estimate, 4);
        adm_data.update_estimate(5);
        assert_eq!(adm_data.estimate, 4);
    }

    #[test]
    fn update_estimate_updates_estimate_if_estimate_is_p() {
        let layer_1 = vec![2, 3, 4, 5].iter().cloned().collect();
        let vias = vec![7, 8, 9].iter().cloned().collect();
        let mut adm_data = AdmData::new(1, layer_1);
        adm_data.estimate = 4;
        adm_data.layer_2.insert(6, vias);
        adm_data.update_estimate(4);
        assert_eq!(adm_data.estimate, 5);
    }

    #[test]
    fn update_direct_neighbour_removes_u_from_layer_1_vertices(){
        let u : Vertex = 2;
        let layer_1 = vec![2, 3, 4, 5,6].iter().cloned().collect();
        let u_layer_1 = vec![7, 8].iter().cloned().collect();
        let expected_layer_1 = vec![3, 4, 5, 6].iter().cloned().collect();
        let mut adm_data = AdmData::new(1, layer_1);
        adm_data.estimate = 5;
        adm_data.update_direct_neighbour(&u,&u_layer_1,3);
        assert_eq!(adm_data.layer_1,expected_layer_1);
        assert_eq!(adm_data.estimate,4);
    }

    #[test]
    fn update_direct_neighbour_adds_each_u_layer_1_to_layer_2_with_u_as_vias(){
        let u : Vertex = 2;
        let layer_1 = vec![2, 3, 4, 5].iter().cloned().collect();
        let u_layer_1: VertexSet = vec![6, 7, 8].iter().cloned().collect();
        let mut expected_layer_2 : VertexMap<VertexSet> = VertexMap::default();
        for v in &u_layer_1{
            let entry = expected_layer_2.entry(*v).or_default();
            entry.insert(u);
        }
        let mut adm_data = AdmData::new(1, layer_1);
        adm_data.estimate = 4;
        adm_data.update_direct_neighbour(&u,&u_layer_1,3);
        assert_eq!(adm_data.layer_2,expected_layer_2);
        assert_eq!(adm_data.estimate,4);
    }

    #[test]
    fn update_direct_neighbour_does_not_add_v_to_layer_2_if_v_is_directly_reachable(){
        let u : Vertex = 2;
        let layer_1: VertexSet = vec![2, 3, 4, 5].iter().cloned().collect();
        let u_layer_1: VertexSet = vec![5, 6, 7, 8].iter().cloned().collect();
        let expected_layer_2_u = u_layer_1.difference(&layer_1);
        let mut expected_layer_2 : VertexMap<VertexSet> = VertexMap::default();
        for v in expected_layer_2_u{
            let entry = expected_layer_2.entry(*v).or_default();
            entry.insert(u);
        }
        let mut adm_data = AdmData::new(1, layer_1);
        adm_data.estimate = 4;
        adm_data.update_direct_neighbour(&u,&u_layer_1,3);
        assert_eq!(adm_data.layer_2,expected_layer_2);
        assert_eq!(adm_data.estimate,4);
    }

    #[test]
    fn update_direct_neighbour_does_not_add_more_than_p_plus_1_vias(){
        let v : Vertex = 6;
        let u : Vertex = 11;
        let layer_1: VertexSet = vec![2, 3, 4, 5].iter().cloned().collect();
        let vias: VertexSet = vec![7,8,9,10].iter().cloned().collect();
        let u_layer_1: VertexSet = vec![6].iter().cloned().collect();
        let mut adm_data = AdmData::new(1, layer_1);
        adm_data.layer_2.insert(v,vias);
        adm_data.estimate = 5;
        adm_data.update_direct_neighbour(&u,&u_layer_1,3);
        let expected = adm_data.layer_2.get(&v).unwrap();
        assert_eq!(expected.len(),4);
        assert_eq!(adm_data.estimate,4);
    }

    #[test]
    fn update_indirect_neighbour_removes_layer_2_vertex(){
        let v : Vertex = 6;
        let layer_1: VertexSet = vec![2, 3, 4, 5].iter().cloned().collect();
        let vias: VertexSet = vec![7,8,9,10].iter().cloned().collect();
        let mut adm_data = AdmData::new(1, layer_1);
        adm_data.layer_2.insert(v,vias);
        adm_data.estimate = 5;
        adm_data.update_indirect_neighbour(&v,3);
        assert_eq!(adm_data.layer_2.contains_key(&v),false);
        assert_eq!(adm_data.estimate,4);
    }
}

#[cfg(test)]
mod test_adm_graph {
    use graphbench::graph::*;
    use graphbench::editgraph::EditGraph;
    use crate::AdmGraph;

    #[test]
    pub fn compute_ordering_returns_true_if_all_v_in_g_has_neighbours_on_or_below_p(){
        let mut graph = EditGraph::new();
        let edges: EdgeSet =  vec![(1, 2), (1, 3), (1,4), (1,5), (2,6), (3,6), (4,6), (5,6)].iter().cloned().collect();
        edges.iter().map(|(u,v)| graph.add_edge(v,u));
        let num_vertices = graph.num_vertices();
        let mut adm_graph = AdmGraph::new(graph);
        assert_eq!(adm_graph.compute_ordering(4),true);
        assert_eq!(adm_graph.unordered.is_empty(),true);
        assert_eq!(adm_graph.ordering.len(),num_vertices);
    }
}

#[cfg(test)]
mod test_main {
    use graphbench::graph::*;
    use graphbench::editgraph::EditGraph;
    use crate::AdmGraph;
    use std::collections::HashMap;
    use rand::thread_rng;
    use rand::prelude::SliceRandom;

    pub fn generate_random_graph() ->EditGraph{
        let mut graph = EditGraph::new();
        let mut nums: Vec<u32> = (20..50).collect();
        nums.shuffle(&mut thread_rng());
        let edges =  vec![
            (1,2), (1,9),
            (2,3), (2,9),
            (3,4), (3,7), (3,9),
            (4,5), (4,6),
            (5,6), (5,8),
            (6,7),
            (7,8),
            (8,9)
        ];
        for (v, u) in edges{
            let new_v = nums[v-1];
            let new_u = nums[u-1];
            graph.add_edge(&new_u,&new_v);
        }
        return graph;
    }

    #[test]
    pub fn test_admissibility_returns_true_for_correct_p_value(){
        let mut graph = generate_random_graph();
        let mut adm_graph = AdmGraph::new(graph);
        assert_eq!(adm_graph.compute_ordering(3),true);
    }

    #[test]
    pub fn test_admissibility_returns_false_for_incorrect_p_value(){
        let mut graph = generate_random_graph();
        let mut adm_graph = AdmGraph::new(graph);
        assert_eq!(adm_graph.compute_ordering(2),false);
    }
}