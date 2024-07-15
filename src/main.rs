mod admGraph;
mod admData;

use clap::{Parser};

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

fn main() {
    let args = Args::parse();
    println!("p is {}", args.p);
}