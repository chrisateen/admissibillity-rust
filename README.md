## 2 Admissibility

Implementation for 2 Admissibility


## To run the program

1. Install the latest version of [rust](https://www.rust-lang.org/tools/install)
2. Download the network-corpus repo and store in a suitable directory
3. Run the following command to run the program on a specific graph
```
cargo run --release <NAME_OF_NETWORK> <P_VALUE_TO_START_THE_SEARCH> <DIR_TO_THE_NETWORK>
```
For example
```
cargo run --release windsurfers 11 ../network-corpus/networks
```
