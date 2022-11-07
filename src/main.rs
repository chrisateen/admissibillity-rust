mod admData;
mod admGraph;
use crate::admData::AdmData;
use std::fs::{OpenOptions};
use clap::Parser;
use std::time::{Instant};
use csv::Writer;
use graphbench::editgraph::EditGraph;
use serde::Serialize;
use crate::admGraph::AdmGraph;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// p
    p: usize,
    /// network file name
    network: String,
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

fn compute(p: usize, network:&String) -> bool{
    let file_dir = format!("../network-corpus/networks/{}.txt.gz", network);
    let graph = EditGraph::from_gzipped(&file_dir).expect(&format!("Error occurred loading graph {}",network));
    let mut adm_graph = AdmGraph::new(graph);
    adm_graph.compute_ordering(p)
}

fn main() {
    let args = Args::parse();
    let mut p = args.p;
    let network = args.network;

    let mut is_p = false;
    let start = Instant::now();
    loop {
        println!("p {}", p);
        is_p = compute(p, &network);
        if is_p { break; }
        p += 1;
    };
    let duration = start.elapsed().as_millis();
    save_result(network, duration, p);
    println!(" Is {0}-2 Admissible:{1}", p, is_p);
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
        let graph = generate_random_graph();
        let mut p = 1;
        let mut is_p = false;
        loop {
            let mut adm_graph = AdmGraph::new(graph.clone());
            is_p = adm_graph.compute_ordering(p);
            if is_p { break; }
            p += 1;
        }
        assert_eq!(p,3);
    }
}