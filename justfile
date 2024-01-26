# Project: euclid
# Script: justfile
#
# A collection of shortcuts to run a series of commands together.

convexhull:
    cargo b
    python tools/sample.py > data/points.txt
    ./target/debug/euclid > data/hull.txt
    python tools/plot.py