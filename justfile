# Project: euclid
# Script: justfile
#
# A collection of shortcuts to run a series of commands together.

convex ALGO PSET:
    cargo b
    python tools/sample.py {{PSET}} 100 > data/points.txt
    ./target/debug/euclid {{ALGO}} data/points.txt data/poly_hull.txt
    python tools/plot.py data/points.txt data/poly_hull.txt


stairs PSET:
    cargo b
    python tools/sample.py {{PSET}} 10 > data/points.txt
    ./target/debug/euclid stairs data/points.txt data/stair.txt
    python tools/plot.py data/points.txt data/stair.txt