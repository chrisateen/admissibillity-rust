mod admData;
mod admGraph;
mod admData2;

use crate::admData::AdmData;
use std::fs::{OpenOptions};
use clap::{Parser};
use std::time::{Instant};
use csv::Writer;
use graphbench::editgraph::EditGraph;
use serde::Serialize;
use crate::admGraph::AdmGraph;

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

#[derive(Debug, Serialize)]
struct Record<'a> {
    network: &'a str,
    duration: u128,
    p: usize
}

fn save_result(network:String, duration: u128, p: usize){
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open("test.csv")
        .unwrap();
    let mut wtr = Writer::from_writer(file);
    wtr.write_record(&[
        network,
        duration.to_string(),
        p.to_string()
    ]);
    wtr.flush();
}

fn load_graph(network_path:String, network:String) -> EditGraph{
    let file_dir = format!("{}/{}.txt.gz", network_path,network);
    EditGraph::from_gzipped(&file_dir).expect(&format!("Error occurred loading graph {}",network))
}

fn compute(p: usize, graph:&EditGraph) -> bool{
    let mut adm_graph = AdmGraph::new(graph);
    adm_graph.compute_ordering(p)
}

fn main() {
    let args = Args::parse();
    let network_path = args.network_path;
    let network = args.network;
    let mut p = args.p;

    let mut is_p;
    let graph = load_graph(network_path, network);
    //let start = Instant::now();
    loop {
        println!("p {}", p);
        is_p = compute(p, &graph);
        if is_p { break; }
        p += 1;
    };
    //let duration = start.elapsed().as_millis();
    //save_result(network, duration, p);
    println!("p is {}", p);
}


#[cfg(test)]
mod test_main {
    use graphbench::graph::*;
    use graphbench::editgraph::EditGraph;
    use crate::AdmGraph;
    use crate::AdmData;
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
    pub fn test_admissibility_returns_correct_p_value(){
        let graph = &generate_random_graph().clone();
        let mut p = 1;
        loop {
            let mut adm_graph = AdmGraph::new(graph);
            let is_p = adm_graph.compute_ordering(p);
            if is_p { break; }
            p += 1;
        }
        assert_eq!(p,3);
    }
}