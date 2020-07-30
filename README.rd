Simple implementation of Conway's Game of Life.
Implementation uses Piston engine for drawing graphics and clap module for passing arguments.
Just put some points to seed.txt and run the binary. Example provides "beacon" and "frog" elements.
Seed file supports comments, starting with "//".
Build instructions:
cargo build

Usage:
./conway --help
./conway --file=seed.txt
