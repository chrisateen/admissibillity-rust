## 2 Admissibility

Implementation for 2 Admissibility

## To run the program

Download the network-corpus repo and store in a suitable directory and ensure you have the latest version of [rust](https://www.rust-lang.org/tools/install) installed

To run the program on a spefic graph run the following command line arguments


'''
cargo run --release <NAME_OF_NETWORK> <P_VALUE_TO_START_THE_SEARCH> <DIR_TO_THE_NETWORK>
'''

For example

'''
carog run --release windsurfers 10  ../network-corpus/networks
'''