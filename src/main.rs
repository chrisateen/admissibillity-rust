#![allow(warnings)]
mod admGraph;
mod augmentingPath;

mod admData;

use crate::admGraph::AdmGraph;
use clap::Parser;
use graphbench::editgraph::EditGraph;
use graphbench::graph::*;
use peak_alloc::PeakAlloc;
use std::cmp::max;

use flate2::write::GzEncoder;
use flate2::Compression;
use std::io::{BufRead, BufWriter, Write};


#[global_allocator]
static PEAK_ALLOC: PeakAlloc = PeakAlloc;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// network file name
    network: String,

    /// start p value
    p: i32,

    /// Path to network
    network_path: String,
}

fn load_graph(network_path: String, network: &String) -> EditGraph {
    let file_dir = format!("{}/{}.txt.gz", network_path, network);
    EditGraph::from_gzipped(&file_dir)
        .unwrap_or_else(|_| panic!("Error occurred loading graph {}", network))
}

fn next_p_value(p: i32, is_p: bool, lowest_p: i32, highest_not_p: i32) -> i32 {
    //Stop where the lowest p is p or the highest p + 1 is p
    if (p - highest_not_p <= 1 && is_p) || (p - lowest_p).abs() == 1 {
        return -1;
    }
    //Continue to double the p value we check if we haven't found a value where G is p,2 admissible
    if lowest_p == -1 && !is_p {
        return (p * 2) as i32;
    }
    //Once we found a p value keep halving the search between the lowest p and the highest not p
    let x = max(p, lowest_p);
    return (x + highest_not_p) / 2;
}

fn compute_ordering(p: usize, graph: &EditGraph) -> Option<Vec<Vertex>> {
    let mut adm_graph = AdmGraph::new(graph);

    adm_graph.initialise_candidates(p);

    // println!("Vertices = {:?}", graph.vertices().collect::<Vec<&Vertex>>());

    let mut next_vertex = adm_graph.remove_v_from_candidates(p);
    let mut order = Vec::default();
    while next_vertex.is_some() && !adm_graph.is_all_vertices_in_r_or_candidates() {
        let v = next_vertex.unwrap();
        order.push(v);
        next_vertex = adm_graph.remove_v_from_candidates(p);
    }
    order.extend(next_vertex.iter()); // Adds vertex if not None

    let found_order = adm_graph.is_all_vertices_in_r_or_candidates();
    if found_order {
        // println!("Next vertex = {:?}", next_vertex);
        // println!("Order = {:?}", order);
        // println!("Cands = {:?}", adm_graph.candidates);
        order.extend(adm_graph.candidates.iter()); 
        assert_eq!(order.len(), graph.num_vertices());
        Some(order)
    } else {
        None
    }
}

fn main() {
    let args = Args::parse();

    let network_path = args.network_path;
    let network = args.network;
    let mut p = args.p;

    let mut lowest_p: i32 = -1;
    let mut highest_not_p: i32 = -1;
    let mut best_order = None;

    let graph = load_graph(network_path, &network);

    let mut peak_mem = PEAK_ALLOC.peak_usage_as_kb();
    println!("Max memory used after graph loading in kb is {}", peak_mem);

    loop {
        let result = compute_ordering(p as usize, &graph);
        let mut found_better = false;
        if let Some(order) = result {
            assert!(lowest_p == -1 || p < lowest_p);
            lowest_p = p;
            best_order = Some(order);
            found_better = true;
        } else {
            assert!(p > highest_not_p);
            highest_not_p = p;
        }

        let next_p = next_p_value(p, found_better, lowest_p, highest_not_p);
        if next_p == -1 {
            if !found_better {
                p = lowest_p;
            }
            break;
        }
        p = next_p;
    }

    println!("p is {}", p);

    peak_mem = PEAK_ALLOC.peak_usage_as_kb();
    println!("Max memory used in total kb is {}", peak_mem);

    if let Some(order) = best_order {
        let current_dir = std::env::current_dir().unwrap();
        let new_folder =  current_dir.join("results");
        std::fs::create_dir_all(&new_folder).unwrap();
        let file_path = new_folder.join(network.as_str().to_owned() + ".txt.gz");

        let file = std::fs::File::create(file_path).unwrap();
        let mut gz = GzEncoder::new(file, Compression::default());

        for v in order {
            writeln!(gz, "{}", v).unwrap();
        }


        gz.finish().unwrap();
    }
}

#[cfg(test)]
mod test_main {

    use crate::compute_ordering;
    use graphbench::editgraph::EditGraph;
    use graphbench::graph::{EdgeSet, MutableGraph};

    #[test]
    pub fn compute_ordering_returns_true_if_all_v_in_g_has_neighbours_on_or_below_p() {
        let mut graph = EditGraph::new();
        let edges: EdgeSet = [
            (1, 2),
            (1, 3),
            (1, 4),
            (1, 5),
            (2, 6),
            (3, 6),
            (4, 6),
            (5, 6),
        ]
        .iter()
        .cloned()
        .collect();
        for (u, v) in edges.iter() {
            graph.add_edge(u, v);
        }

        assert!(compute_ordering(4, &graph).is_some());
    }

    #[test]
    pub fn compute_ordering_returns_true_for_correct_p_value() {
        let mut graph = EditGraph::new();
        let edges: EdgeSet = [(1, 2), (1, 3), (1, 4), (2, 3), (2, 4), (3, 4)]
            .iter()
            .cloned()
            .collect();
        for (u, v) in edges.iter() {
            graph.add_edge(u, v);
        }

        assert!(compute_ordering(4, &graph).is_some());
    }

    #[test]
    pub fn compute_ordering_returns_false_for_incorrect_p_value() {
        let mut graph = EditGraph::new();
        let edges: EdgeSet = [(1, 2), (1, 3), (1, 4), (2, 3), (2, 4), (3, 4)]
            .iter()
            .cloned()
            .collect();
        for (u, v) in edges.iter() {
            graph.add_edge(u, v);
        }

        assert!(!compute_ordering(2, &graph).is_some());
    }

    #[test]
    pub fn test_admissibility_returns_correct_p_value() {
        let mut graph = EditGraph::new();
        let edges: EdgeSet = [
            (1, 2),
            (1, 9),
            (2, 3),
            (2, 9),
            (3, 4),
            (3, 7),
            (3, 9),
            (4, 5),
            (4, 6),
            (5, 6),
            (5, 8),
            (6, 7),
            (7, 8),
            (8, 9),
        ]
        .iter()
        .cloned()
        .collect();
        for (u, v) in edges.iter() {
            graph.add_edge(u, v);
        }

        let mut p = 1;
        loop {
            let is_p = compute_ordering(p, &graph);
            if is_p.is_some() {
                break;
            }
            p += 1;
        }

        assert_eq!(p, 3);
    }
}
