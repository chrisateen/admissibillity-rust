## 2 Admissibility

Implementation for 2 Admissibility


## To run the program

1. Binaries are available for windows (x86) linux (x86 and ARM) and macOS (x86 and ARM) in GitHub releases
2. This program can be run using the collection of networks found in the [network-corpus repo](https://github.com/microgravitas/network-corpus)
3. Download the [network-corpus repo](https://github.com/microgravitas/network-corpus) and run the following command to run the program on a specific network
```
admissibility-rust <NAME_OF_NETWORK> <P_VALUE_TO_START_THE_SEARCH> <DIR_TO_THE_NETWORK>
```
For example
```
admissibility-rust windsurfers 11 ../network-corpus/networks
```
### Saving ordering to a file
The 2-admissibility ordering for a graph can be saved to a txt.gz file by using the save command.
```
admissibility-rust <NAME_OF_NETWORK> <P_VALUE_TO_START_THE_SEARCH> <DIR_TO_THE_NETWORK> save <OPTIONAL_DIR_WHERE_TO_SAVE_THE_FILE>
```
For example the below would save orderings for the graph in /Users/username/orderings/windsurfers.txt.gz
```
admissibility-rust windsurfers 11 ../network-corpus/networks /Users/username/orderings
```
If no directory is included a results directory will be created in the directory where the program is running and save the results there

