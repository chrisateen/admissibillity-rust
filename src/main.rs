mod admGraph;
mod augmentingPath;

use crate::admGraph::AdmGraph;
use clap::Parser;
use graphbench::editgraph::EditGraph;
use graphbench::graph::{Graph, VertexSet};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// network file name
    network: String,

    #[arg(default_value_t = String::from("../network-corpus/networks/"))]
    network_path: String,

    //start p value
    #[arg(short, long, default_value_t = 1)]
    p: usize,
}

fn load_graph(network_path: String, network: String) -> EditGraph {
    let file_dir = format!("{}/{}.txt.gz", network_path, network);
    EditGraph::from_gzipped(&file_dir)
        .unwrap_or_else(|_| panic!("Error occurred loading graph {}", network))
}

fn compute_ordering(p: usize, graph: &EditGraph) -> bool {
    let num_vertices = graph.num_vertices();
    let mut adm_graph = AdmGraph::new(&graph);
    let mut ordering = VertexSet::default();

    adm_graph.initialise_candidates(p);

    let mut next_vertex = adm_graph.remove_v_from_candidates(p);

    while next_vertex.is_some() {
        ordering.insert(next_vertex.unwrap());
        next_vertex = adm_graph.remove_v_from_candidates(p);
    }

    ordering.len() == num_vertices
}

fn main() {
    let args = Args::parse();

    let network_path = args.network_path;
    let network = args.network;
    let mut p = args.p;

    let mut is_p;

    let graph = load_graph(network_path, network);

    loop {
        println!("p {}", p);
        is_p = compute_ordering(p, &graph);
        if is_p {
            break;
        }
        p += 1;
    }

    println!("p is {}", p);
}

#[cfg(test)]
mod test_main {
    use crate::admGraph::AdmGraph;
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

        assert!(compute_ordering(4, &graph));
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

        assert!(compute_ordering(4, &graph));
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

        assert!(!compute_ordering(2, &graph));
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
            let mut adm_graph = AdmGraph::new(&graph);
            let is_p = compute_ordering(p, &graph);
            if is_p {
                break;
            }
            p += 1;
        }

        assert_eq!(p, 3);
    }
}
