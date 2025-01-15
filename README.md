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
### Saving ordering to a file
The 2-admissibility ordering for a graph can be saved to a txt.gz file by using the save command.
```
cargo run --release <NAME_OF_NETWORK> <P_VALUE_TO_START_THE_SEARCH> <DIR_TO_THE_NETWORK> save <OPTIONAL_DIR_WHERE_TO_SAVE_THE_FILE>
```
For example the below would save orderings for the graph in /Users/username/orderings/windsurfers.txt.gz
```
cargo run --release windsurfers 11 ../network-corpus/networks /Users/username/orderings
```
If no directory is included a results directory will be created in the directory where the program is running and save the results there

