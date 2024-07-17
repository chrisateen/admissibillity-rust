mod admGraph;
mod augmentingPath;

mod admData;

use std::cmp::{max, Ordering};
use crate::admGraph::AdmGraph;
use clap::Parser;
use graphbench::editgraph::EditGraph;
use graphbench::graph::{Graph, VertexSet, Vertex};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// network file name
    network: String,

    /// start p value
    p: usize,

    /// Path to network
    #[arg(default_value_t = String::from("../network-corpus/networks/"))]
    network_path: String,
}

struct AdmResult {
    ordering: Vec<Vertex>,
    is_p: bool
}

fn load_graph(network_path: String, network: String) -> EditGraph {
    let file_dir = format!("{}/{}.txt.gz", network_path, network);
    EditGraph::from_gzipped(&file_dir)
        .unwrap_or_else(|_| panic!("Error occurred loading graph {}", network))
}

fn compute_ordering(p: usize, graph: &EditGraph, previous_ordering: &Vec<Vertex>) -> AdmResult {
    let mut adm_graph = AdmGraph::new(graph);
    let mut result  = AdmResult{
        is_p: false,
        ordering: Vec::new()
    };

    adm_graph.initialise_candidates(p);

    if !previous_ordering.is_empty(){
        adm_graph.initialise_from_previous_iteration(p,previous_ordering);
        result.ordering.extend(previous_ordering);
    }

    let mut next_vertex = adm_graph.remove_v_from_candidates(p, None);

    while next_vertex.is_some() && !adm_graph.is_all_vertices_in_r_or_candidates() {
        result.ordering.push(next_vertex.unwrap());
        next_vertex = adm_graph.remove_v_from_candidates(p, None);
    }

    result.is_p = adm_graph.is_all_vertices_in_r_or_candidates();

    result
}

fn main() {
    let args = Args::parse();

    let network_path = args.network_path;
    let network = args.network;
    let mut p = args.p;

    let mut is_p;

    let mut previous_ordering = Vec::new();

    let graph = load_graph(network_path, network);
    loop {
        println!("p {}", p);
        let result = compute_ordering(p, &graph, &previous_ordering);
        is_p = result.is_p;
        previous_ordering = result.ordering;
        if is_p {
            break;
        }
        p += 1;
    }

    println!("p is {}", p);
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

        assert!(compute_ordering(4, &graph, &Vec::new()).is_p);
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

        assert!(compute_ordering(4, &graph, &Vec::new()).is_p);
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

        assert!(!compute_ordering(2, &graph, &Vec::new()).is_p);
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
            let result = compute_ordering(p, &graph, &Vec::new());
            if result.is_p {
                break;
            }
            p += 1;
        }

        assert_eq!(p, 3);
    }
}
